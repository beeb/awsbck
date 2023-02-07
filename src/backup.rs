#![allow(unused)] // TODO: remove
use std::ffi::OsStr;
use std::{collections::HashMap, fs::File, path::Path, time::SystemTime};

use anyhow::{anyhow, Context, Result};
use dropbox_sdk::oauth2::Authorization;
use dropbox_sdk::{default_client::UserAuthDefaultClient, files};
use flate2::write::GzEncoder;
use flate2::Compression;

/// The size of a block. This is a Dropbox constant, not adjustable.
const BLOCK_SIZE: usize = 4 * 1024 * 1024;

/// Perform a backup of the folder, uploading it to Dropbox once complete.
///
/// `folder` is the path to the folder to backup, and `token` is the Dropbox generated access token.
pub fn backup(folder: &Path, token: impl Into<String>) -> Result<()> {
    let token: String = token.into();
    let folder_name = folder
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or("volume".to_string());
    let (archive, size, mtime) =
        compress_folder(folder).with_context(|| anyhow!("compression failed"))?;
    println!("Archive size: {size} bytes");
    let auth = Authorization::from_access_token(token);
    let client = UserAuthDefaultClient::new(auth);
    Ok(())
}

fn compress_folder(folder: &Path) -> Result<(File, u64, SystemTime)> {
    let tar_gz: File = tempfile::tempfile()?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", folder)?;
    let res = tar.into_inner()?;
    let tar_gz = res.finish()?;
    let meta = tar_gz.metadata()?;
    Ok((tar_gz, meta.len(), meta.modified()?))
}

#[derive(Default)]
struct CompletionTracker {
    complete_up_to: u64,
    uploaded_blocks: HashMap<u64, u64>,
}

impl CompletionTracker {
    fn complete_block(&mut self, block_offset: u64, block_len: u64) {
        if block_offset == self.complete_up_to {
            self.complete_up_to += block_len;
            while let Some(len) = self.uploaded_blocks.remove(&self.complete_up_to) {
                self.complete_up_to += len;
            }
        } else {
            self.uploaded_blocks.insert(block_offset, block_len);
        }
    }
}

#[derive(Default)]
struct UploadSession {
    session_id: String,
    start_offset: u64,
    file_size: u64,
    bytes_transferred: u64,
    completion: CompletionTracker,
}

impl UploadSession {
    fn new(client: &UserAuthDefaultClient, file_size: u64) -> Result<Self> {
        let session_id = match files::upload_session_start(
            client,
            &files::UploadSessionStartArg::default()
                .with_session_type(files::UploadSessionType::Concurrent),
            &[],
        ) {
            Ok(Ok(result)) => result.session_id,
            err => return Err(anyhow!("Starting upload session failed: {err:?}")),
        };

        Ok(Self {
            session_id,
            file_size,
            ..Default::default()
        })
    }
    fn append_arg(&self, block_offset: u64) -> files::UploadSessionAppendArg {
        files::UploadSessionAppendArg::new(files::UploadSessionCursor::new(
            self.session_id.clone(),
            self.start_offset + block_offset,
        ))
    }
    fn commit_arg(
        &self,
        dest_path: String,
        source_mtime: SystemTime,
    ) -> Result<files::UploadSessionFinishArg> {
        Ok(files::UploadSessionFinishArg::new(
            files::UploadSessionCursor::new(self.session_id.clone(), self.file_size),
            files::CommitInfo::new(dest_path).with_client_modified(iso8601(source_mtime)?),
        ))
    }
    fn mark_block_uploaded(&mut self, block_offset: u64, block_len: u64) {
        self.completion
            .complete_block(self.start_offset + block_offset, block_len);
    }
    fn complete_up_to(&self) -> u64 {
        self.completion.complete_up_to
    }
}

fn get_file_mtime_and_size(f: &File) -> Result<(SystemTime, u64)> {
    let meta = f
        .metadata()
        .with_context(|| anyhow!("Error getting source file metadata"))?;
    let mtime = meta
        .modified()
        .with_context(|| anyhow!("Error getting source file mtime"))?;
    Ok((mtime, meta.len()))
}

fn iso8601(t: SystemTime) -> Result<String> {
    let timestamp: i64 = match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    };

    Ok(
        chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0 /* nsecs */)
            .ok_or_else(|| anyhow!("invalid or out-of-range timestamp"))?
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string(),
    )
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use dotenvy::dotenv;
    use dropbox_sdk::UserAuthClient;

    use super::*;

    #[ctor::ctor]
    fn init() {
        dotenv().ok();
    }

    #[test]
    fn test_token() {
        let token = std::env::var("DOCKERBOX_TOKEN").unwrap();
        let auth = Authorization::from_access_token(token);
        let client = UserAuthDefaultClient::new(auth);
        let iter = match files::list_folder(&client, &files::ListFolderArg::new(String::new())) {
            Ok(Ok(result)) => {
                let cursor = if result.has_more {
                    Some(result.cursor)
                } else {
                    None
                };

                Ok(Ok(DirectoryIterator {
                    client: &client,
                    cursor,
                    buffer: result.entries.into(),
                }))
            }
            Ok(Err(err)) => Ok(Err(err)),
            Err(err) => Err(err),
        };
        match iter {
            Ok(Ok(iter)) => {
                for entry_res in iter {
                    match entry_res {
                        Ok(Ok(files::Metadata::Folder(entry))) => {
                            println!("Folder: {}", entry.path_display.unwrap_or(entry.name));
                        }
                        Ok(Ok(files::Metadata::File(entry))) => {
                            println!("File: {}", entry.path_display.unwrap_or(entry.name));
                        }
                        Ok(Ok(files::Metadata::Deleted(entry))) => {
                            eprintln!("unexpected deleted entry: {entry:?}");
                        }
                        Ok(Err(e)) => {
                            eprintln!("Error from files/list_folder_continue: {e}");
                            break;
                        }
                        Err(e) => {
                            eprintln!("API request error: {e}");
                            break;
                        }
                    }
                }
            }
            Ok(Err(err)) => eprintln!("API error: {err:?}"),
            Err(err) => eprintln!("API error: {err:?}"),
        }
    }

    struct DirectoryIterator<'a, T: UserAuthClient> {
        client: &'a T,
        buffer: VecDeque<files::Metadata>,
        cursor: Option<String>,
    }

    impl<'a, T: UserAuthClient> Iterator for DirectoryIterator<'a, T> {
        type Item = dropbox_sdk::Result<Result<files::Metadata, files::ListFolderContinueError>>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(entry) = self.buffer.pop_front() {
                Some(Ok(Ok(entry)))
            } else if let Some(cursor) = self.cursor.take() {
                match files::list_folder_continue(
                    self.client,
                    &files::ListFolderContinueArg::new(cursor),
                ) {
                    Ok(Ok(result)) => {
                        self.buffer.extend(result.entries.into_iter());
                        if result.has_more {
                            self.cursor = Some(result.cursor);
                        }
                        self.buffer.pop_front().map(|entry| Ok(Ok(entry)))
                    }
                    Ok(Err(e)) => Some(Ok(Err(e))),
                    Err(e) => Some(Err(e)),
                }
            } else {
                None
            }
        }
    }
}
