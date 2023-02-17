use std::path::PathBuf;
use std::{fs::File, path::Path};

use anyhow::{anyhow, Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use temp_dir::TempDir;
use uuid::Uuid;

use crate::{aws::upload_file, config::Params};

/// A compressed archive of a folder.
///
/// The `_temp_dir` is not used but needs to be kept around until the upload is complete. It going out of scope will
/// delete the temp folder.
pub(crate) struct Archive {
    pub(crate) path: PathBuf,
    _temp_dir: TempDir,
}

/// Perform a backup of the folder, uploading it to Dropbox once complete.
pub(crate) async fn backup(params: &Params) -> Result<()> {
    let archive = compress_folder(&params.folder).with_context(|| anyhow!("compression failed"))?;
    upload_file(archive, params)
        .await
        .with_context(|| anyhow!("upload failed"))?;
    Ok(())
}

/// Compress the folder into a randomly named tar.gz archive in a temp directory
fn compress_folder(folder: impl AsRef<Path>) -> Result<Archive> {
    // create a temp directory, it will be deleted when the ref goes out of scope
    let dir = TempDir::new()?;
    // generate a random filename
    let filename = format!("{}.tar.gz", Uuid::new_v4());
    let file_path = dir.child(filename);
    // create the file handle
    let tar_gz: File = File::create(&file_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    // insert the contents of folder into the archive, recursively, at the root of the archive
    // note that the folder itself is not present in the archive, only its contents
    tar.append_dir_all(".", folder)?;
    // make sure the tar layer data is written
    let res = tar.into_inner()?;
    // make sure the gz layer data is written
    res.finish()?;
    // we keep temp dir reference to avoid premature deletion
    Ok(Archive {
        path: file_path,
        _temp_dir: dir,
    })
}
