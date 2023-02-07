//#![allow(unused)]
use std::env;

use std::{fs::File, path::Path};

use anyhow::{anyhow, Context, Result};
use aws_sdk_s3::model::CompletedMultipartUpload;
use aws_sdk_s3::{
    model::CompletedPart, output::CreateMultipartUploadOutput, types::ByteStream, Client,
};
use aws_smithy_http::byte_stream::Length;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::config::Params;

//In bytes, minimum chunk size of 5MB. Increase CHUNK_SIZE to send larger chunks.
const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

/// Perform a backup of the folder, uploading it to Dropbox once complete.
pub async fn backup(params: &Params) -> Result<()> {
    let archive_name =
        compress_folder(&params.folder).with_context(|| anyhow!("compression failed"))?;
    upload_file(archive_name, params)
        .await
        .with_context(|| anyhow!("upload failed"))?;
    Ok(())
}

fn compress_folder(folder: &Path) -> Result<String> {
    let filename = "dockerbck.tar.gz";
    let tar_gz: File = File::create(filename)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", folder)?;
    let res = tar.into_inner()?;
    res.finish()?;
    Ok(filename.to_string())
}

fn get_file_size(archive_name: impl Into<String>) -> Result<u64> {
    let file = File::open(archive_name.into())?;
    let metadata = file.metadata()?;
    Ok(metadata.len())
}

async fn upload_file(archive_name: impl Into<String>, params: &Params) -> Result<()> {
    let archive_name = archive_name.into();
    env::set_var("AWS_ACCESS_KEY_ID", &params.aws_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", &params.aws_key);
    let shared_config = aws_config::from_env()
        .region(params.aws_region.region().await)
        .load()
        .await;
    let client = Client::new(&shared_config);
    let filename = format!(
        "dockerbck_{}.tar.gz",
        params
            .folder
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or("volume".to_string())
    );
    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(&params.aws_bucket)
        .key(&filename)
        .send()
        .await?;
    let upload_id = multipart_upload_res
        .upload_id()
        .ok_or_else(|| anyhow!("upload_id not found"))?;
    let file_size = get_file_size(&archive_name)?;
    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }

    if file_size == 0 {
        return Err(anyhow!("file size is 0"));
    }
    if chunk_count > MAX_CHUNKS {
        return Err(anyhow!("too many chunks, try increasing the chunk size"));
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = match chunk_index {
            i if i == chunk_count - 1 => size_of_last_chunk,
            _ => CHUNK_SIZE,
        };
        let stream = ByteStream::read_from()
            .path(&archive_name)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await?;

        let part_number = (chunk_index as i32) + 1;
        let upload_part_res = client
            .upload_part()
            .key(&filename)
            .bucket(&params.aws_bucket)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await?;
        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
    }
    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();
    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(&params.aws_bucket)
        .key(&filename)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await?;
    Ok(())
}
