use std::path::PathBuf;
use std::{fs::File, path::Path};

use anyhow::{anyhow, Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use temp_dir::TempDir;
use uuid::Uuid;

use crate::{aws::upload_file, config::Params};

/// Perform a backup of the folder, uploading it to Dropbox once complete.
pub async fn backup(params: &Params) -> Result<()> {
    let (archive_path, temp_dir) =
        compress_folder(&params.folder).with_context(|| anyhow!("compression failed"))?;
    upload_file(archive_path, temp_dir, params)
        .await
        .with_context(|| anyhow!("upload failed"))?;
    Ok(())
}

fn compress_folder(folder: &Path) -> Result<(PathBuf, TempDir)> {
    let dir = TempDir::new()?;
    let filename = format!("{}.tar.gz", Uuid::new_v4());
    let file_path = dir.child(filename);
    let tar_gz: File = File::create(&file_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", folder)?;
    let res = tar.into_inner()?;
    res.finish()?;
    Ok((file_path, dir))
}
