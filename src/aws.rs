//! AWS S3 upload module
//!
//! This was adapted from the
//! [official examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/examples/s3)
use std::{env, fs::File, path::Path};

use anyhow::{anyhow, Result};
use aws_sdk_s3::{
    operation::create_multipart_upload::CreateMultipartUploadOutput,
    types::{CompletedMultipartUpload, CompletedPart},
    Client,
};
use aws_smithy_types::byte_stream::{ByteStream, Length};
use log::{info, warn};

use crate::{
    backup::Archive,
    config::{sanitize_filename, Params},
};

/// In bytes, minimum chunk size of 5MB. Increase [`CHUNK_SIZE`] to send larger chunks.
const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

/// Upload a file to AWS S3 using multipart.
///
/// The `_temp_dir` is not used but needs to be kept around until the upload is complete. It going out of scope will
/// delete the temp folder.
pub(crate) async fn upload_file(archive: Archive, params: &Params) -> Result<()> {
    // we want to use `from_env` below, so make sure that environment variables are set properly, even if data comes
    // from the command line args
    env::set_var("AWS_ACCESS_KEY_ID", &params.aws_key_id);
    env::set_var("AWS_SECRET_ACCESS_KEY", &params.aws_key);
    let mut shared_config_builder = aws_config::from_env().region(params.aws_region.region().await);
    // we set this special environment variable when doing e2e testing
    if env::var("AWSBCK_TESTING_E2E").is_ok() {
        warn!("Endpoint URL was changed to localhost while in testing environment.");
        shared_config_builder = shared_config_builder.endpoint_url("http://127.0.0.1:9090");
    }
    let shared_config = shared_config_builder.load().await;
    let client = Client::new(&shared_config);
    // if the desired filename was specified, append the file extension in case it was not already provided
    let filename = params.filename.clone().map_or_else(
        || {
            // default filename is awsbck_ + the folder name + .tar.gz
            let sanitized_folder_name =
                params.folder.file_name().map_or("backup".to_string(), |f| {
                    sanitize_filename(f.to_string_lossy().to_string())
                });
            format!("awsbck_{sanitized_folder_name}.tar.gz")
        },
        |f| match f {
            f if !f.ends_with(".tar.gz") => format!("{f}.tar.gz"),
            f => f,
        },
    );
    let multipart_upload_res: CreateMultipartUploadOutput = client
        .create_multipart_upload()
        .bucket(&params.aws_bucket)
        .key(&filename)
        .send()
        .await?;
    let upload_id = multipart_upload_res
        .upload_id()
        .ok_or_else(|| anyhow!("upload_id not found"))?; // convert option to error if None
    let file_size = get_file_size(&archive.path)?;
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
            .path(&archive.path)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await?;

        // this should be a uint but somehow the API expects an i32 (which starts at 1)
        #[allow(clippy::cast_possible_truncation)] // between 1 and 10_000
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

    info!("Archive uploaded as \"{}\"", &filename);
    Ok(())
}

/// Utility function to get the file size
fn get_file_size(archive_path: impl AsRef<Path>) -> Result<u64> {
    let file = File::open(archive_path)?;
    let metadata = file.metadata()?;
    Ok(metadata.len())
}
