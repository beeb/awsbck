use std::{env, fs::File, path::PathBuf};

use anyhow::{anyhow, Result};
use aws_sdk_s3::{
    model::{CompletedMultipartUpload, CompletedPart},
    output::CreateMultipartUploadOutput,
    types::ByteStream,
    Client,
};
use aws_smithy_http::byte_stream::Length;
use temp_dir::TempDir;

use crate::config::Params;

/// In bytes, minimum chunk size of 5MB. Increase CHUNK_SIZE to send larger chunks.
const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

pub async fn upload_file(archive_path: PathBuf, _temp_dir: TempDir, params: &Params) -> Result<()> {
    env::set_var("AWS_ACCESS_KEY_ID", &params.aws_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", &params.aws_key);
    let shared_config = aws_config::from_env()
        .region(params.aws_region.region().await)
        .load()
        .await;
    let client = Client::new(&shared_config);
    let filename = format!(
        "awsbck_{}.tar.gz",
        params
            .folder
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or("backup".to_string())
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
    let file_size = get_file_size(&archive_path)?;
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
            .path(&archive_path)
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

fn get_file_size(archive_path: &PathBuf) -> Result<u64> {
    let file = File::open(archive_path)?;
    let metadata = file.metadata()?;
    Ok(metadata.len())
}
