use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Region;
use clap::{command, Parser};
use log::*;

use crate::prelude::*;

/// https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html
const VALID_FILENAME_CHARS: &str = "!-_.*'()/"; // plus alphanumeric

/// CLI Parser uses `clap`.
///
/// command args take precedence over environment variables.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the folder to backup
    #[arg(value_hint = clap::ValueHint::DirPath, env = "AWSBCK_FOLDER")]
    folder: Option<PathBuf>,

    /// Specify an interval in seconds to run the backup periodically
    ///
    /// If not specified, the backup will only run once
    #[arg(short, long, value_name = "SECONDS", env = "AWSBCK_INTERVAL")]
    interval: Option<u64>,

    /// The name of the archive that will be uploaded to S3, without extension (optional)
    #[arg(short, long, value_name = "NAME", env = "AWSBCK_FILENAME")]
    filename: Option<String>,

    /// The AWS S3 region
    #[arg(
        short = 'r',
        long = "region",
        value_name = "REGION",
        env = "AWS_REGION"
    )]
    aws_region: Option<String>,

    /// The AWS S3 bucket name
    #[arg(
        short = 'b',
        long = "bucket",
        value_name = "BUCKET",
        env = "AWS_BUCKET"
    )]
    aws_bucket: Option<String>,

    /// The AWS S3 access key ID
    #[arg(long = "id", value_name = "KEY_ID", env = "AWS_ACCESS_KEY_ID")]
    aws_key_id: Option<String>,

    /// The AWS S3 secret access key
    #[arg(
        short = 'k',
        long = "key",
        value_name = "KEY",
        env = "AWS_SECRET_ACCESS_KEY"
    )]
    aws_key: Option<String>,
}

/// Runtime parameters, parsed, validated and ready to be used
pub(crate) struct Params {
    /// Which folder to backup
    pub(crate) folder: PathBuf,
    /// An optional interval duration in seconds
    pub(crate) interval: Option<u64>,
    /// The optional name of the archive that will be uploaded to S3 (without extension)
    pub(crate) filename: Option<String>,
    /// The AWS S3 region, defaults to us-east-1
    pub(crate) aws_region: RegionProviderChain,
    /// The AWS S3 bucket name
    pub(crate) aws_bucket: String,
    /// The AWS S3 access key ID
    pub(crate) aws_key_id: String,
    /// The AWS S3 access key
    pub(crate) aws_key: String,
}

/// Parse the command-line arguments and environment variables into runtime params
pub(crate) async fn parse_config() -> Result<Params> {
    // Read from the command-line args, and if not present, check environment variables
    let params = Cli::parse();

    // ensure we have all the required parameters
    let Some(folder) = params.folder else {
        return Err(anyhow!("No folder path was provided"));
    };
    // make sure folder exists
    let folder = folder
        .canonicalize()
        .with_context(|| anyhow!("Could not resolve path {}", folder.to_string_lossy()))?;
    if !folder.is_dir() {
        return Err(anyhow!("'{}' is not a folder", folder.to_string_lossy()));
    }
    // all AWS parameters are required
    let aws_region = RegionProviderChain::first_try(params.aws_region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    info!("Using AWS region: {}", aws_region.region().await.or_panic());
    let Some(aws_bucket) = params.aws_bucket else {
        return Err(anyhow!("No AWS bucket name was provided"));
    };
    let Some(aws_key_id) = params.aws_key_id else {
        return Err(anyhow!("No AWS key ID was provided"));
    };
    let Some(aws_key) = params.aws_key else {
        return Err(anyhow!("No AWS secret key was provided"));
    };

    // sanitize filename
    let filename = params
        .filename
        .map(sanitize_filename)
        .filter(|s| !s.is_empty()); // if only bad chars were provided, ignore and use default filename

    Ok(Params {
        folder,
        interval: params.interval,
        filename,
        aws_region,
        aws_bucket,
        aws_key_id,
        aws_key,
    })
}

/// Only keep recommended chars for S3 object keys and truncate to 1000 chars
fn sanitize_filename(filename: impl Into<String>) -> String {
    let mut filename: String = filename.into();
    // remove invalid characters
    filename.retain(|c| c.is_ascii_alphanumeric() || VALID_FILENAME_CHARS.contains(c));
    // since we only have ascii and single-byte special chars, we should be able to keep 1024 chars to stay under
    // 1024 bytes, but for good measure we'll limit to 1000
    filename.chars().take(1000).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(&sanitize_filename("foo123"), "foo123");
        assert_eq!(&sanitize_filename("foo bar"), "foobar");
        assert_eq!(&sanitize_filename("foo/bar"), "foo/bar");
        assert_eq!(&sanitize_filename("foo.tar.gz"), "foo.tar.gz");
        assert_eq!(&sanitize_filename("٣৬¾①🦀"), "");
        assert_eq!(&sanitize_filename("!-_.*'()/"), "!-_.*'()/");
        assert_eq!(sanitize_filename("Bar1".repeat(256)), "Bar1".repeat(250));
    }
}
