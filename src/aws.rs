//! AWS S3 upload module
//!
//! This was adapted from the
//! [official examples](https://github.com/awslabs/aws-sdk-rust/blob/main/examples/s3/README.md)
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

/// Upload a file to AWS S3 using multipart.
///
/// The `_temp_dir` is not used but needs to be kept around until the upload is complete. It going out of scope will
/// delete the temp folder.
pub async fn upload_file(archive_path: PathBuf, _temp_dir: TempDir, params: &Params) -> Result<()> {
    // we want to use `from_env` below, so make sure that environment variables are set properly, even if data comes
    // from the command line args
    env::set_var("AWS_ACCESS_KEY_ID", &params.aws_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", &params.aws_key);
    let shared_config = aws_config::from_env()
        .region(params.aws_region.region().await) // set the region
        .load()
        .await;
    let client = Client::new(&shared_config);
    // if the desired filename was specified, append the file extension in case it was not already provided
    let filename = params
        .filename
        .clone()
        .map(|f| match f {
            f if !f.ends_with(".tar.gz") => format!("{f}.tar.gz"),
            f => f,
        })
        .unwrap_or_else(|| {
            // default filename is awsbck_ + the folder name + .tar.gz
            format!(
                "awsbck_{}.tar.gz",
                params
                    .folder
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or("backup".to_string())
            )
        });
    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(&params.aws_bucket)
        .key(&filename)
        .send()
        .await?;
    let upload_id = multipart_upload_res
        .upload_id()
        .ok_or_else(|| anyhow!("upload_id not found"))?; // convert option to error if None
    let file_size = get_file_size(&archive_path)?;
    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    // if the file size is exactly a multiple of CHUNK_SIZE, we don't need the extra chunk
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }
    // something went very wrong if we get a size of zero here
    if file_size == 0 {
        return Err(anyhow!("file size is 0"));
    }
    // AWS will not accept an upload with too many chunks
    if chunk_count > MAX_CHUNKS {
        return Err(anyhow!("too many chunks, try increasing the chunk size"));
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    // upload all chunks
    for chunk_index in 0..chunk_count {
        let this_chunk = match chunk_index {
            i if i == chunk_count - 1 => size_of_last_chunk,
            _ => CHUNK_SIZE,
        };
        // take the relevant part of the file corresponding to this chunk
        let stream = ByteStream::read_from()
            .path(&archive_path)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await?;

        // this should be a uint but somehow the API expects an i32 (which starts at 1)
        let part_number = (chunk_index as i32) + 1;

        // send chunk and record the ETag and part number
        let upload_part_res = client
            .upload_part()
            .key(&filename)
            .bucket(&params.aws_bucket)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await?;

        // this vec of chunks is required to finalize the upload
        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
    }
    // complete the upload
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

/// Utility function to get the file size
fn get_file_size(archive_path: &PathBuf) -> Result<u64> {
    let file = File::open(archive_path)?;
    let metadata = file.metadata()?;
    Ok(metadata.len())
}
