//#![allow(unused)]
use std::env;

use std::time::Duration;
use std::{fs::File, path::Path};

use anyhow::{anyhow, Context, Result};
use aws_sdk_s3::{presigning::config::PresigningConfig, types::ByteStream, Client};
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::config::Params;

/// Perform a backup of the folder, uploading it to Dropbox once complete.
pub async fn backup(params: &Params) -> Result<()> {
    let archive = compress_folder(&params.folder).with_context(|| anyhow!("compression failed"))?;
    upload_file(archive, params)
        .await
        .with_context(|| anyhow!("upload failed"))?;
    Ok(())
}

fn compress_folder(folder: &Path) -> Result<File> {
    let tar_gz: File = tempfile::tempfile()?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", folder)?;
    let res = tar.into_inner()?;
    let tar_gz = res.finish()?;
    Ok(tar_gz)
}

async fn upload_file(file: File, params: &Params) -> Result<()> {
    env::set_var("AWS_ACCESS_KEY_ID", &params.aws_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", &params.aws_key);
    let shared_config = aws_config::from_env()
        .region(params.aws_region.region().await)
        .load()
        .await;
    let client = Client::new(&shared_config);
    let expires_in = Duration::from_secs(60);
    client
        .put_object()
        .bucket(&params.aws_bucket)
        .key(format!(
            "dockerbck_{}.tar.gz",
            params
                .folder
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or("volume".to_string())
        ))
        .body(
            ByteStream::read_from()
                .file(tokio::fs::File::from_std(file))
                .build()
                .await?,
        )
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    Ok(())
}
