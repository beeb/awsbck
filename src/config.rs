use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Region;
use clap::{command, Parser};
use cron::Schedule;

use crate::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the folder to backup
    #[arg(value_hint = clap::ValueHint::DirPath, env = "DOCKERBCK_FOLDER")]
    folder: Option<PathBuf>,

    /// Specify a cron expression to run the backup periodically
    ///
    /// If not specified, the backup will only run once
    #[arg(short, long, value_name = "CRON", env = "DOCKERBCK_SCHEDULE")]
    schedule: Option<String>,

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
    #[arg(
        short = 'i',
        long = "id",
        value_name = "KEY_ID",
        env = "AWS_ACCESS_KEY_ID"
    )]
    aws_key_id: Option<String>,

    /// The AWS S3 secret access key
    #[arg(
        short = 't',
        long = "token",
        value_name = "KEY",
        env = "AWS_SECRET_ACCESS_KEY"
    )]
    aws_key: Option<String>,
}

/// Runtime parameters, parsed and ready to be used
pub struct Params {
    /// Which folder to backup
    pub folder: PathBuf,
    /// An optional parsed cron expression
    pub schedule: Option<Schedule>,
    /// The AWS S3 region
    pub aws_region: RegionProviderChain,
    /// The AWS S3 bucket name
    pub aws_bucket: String,
    /// The AWS S3 access key ID
    pub aws_key_id: String,
    /// The AWS S3 access key
    pub aws_key: String,
}

/// Parse the command-line arguments and environment variables into runtime params
pub async fn parse_config() -> Result<Params> {
    let params = Cli::parse();

    let Some(folder) = params.folder else {
        return Err(anyhow!("No folder path was provided"));
    };
    let folder = folder
        .canonicalize()
        .with_context(|| anyhow!("Could not resolve path {}", folder.to_string_lossy()))?;
    if !folder.is_dir() {
        return Err(anyhow!("'{}' is not a folder", folder.to_string_lossy()));
    }

    let schedule: Option<Schedule> = match params.schedule {
        Some(s) => Some(
            s.parse()
                .with_context(|| anyhow!("Could not parse cron expression"))?,
        ),
        None => None,
    };

    let aws_region = RegionProviderChain::first_try(params.aws_region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    println!("Using AWS region: {}", aws_region.region().await.or_panic());
    let Some(aws_bucket) = params.aws_bucket else {
        return Err(anyhow!("No AWS bucket name was provided"));
    };
    let Some(aws_key_id) = params.aws_key_id else {
        return Err(anyhow!("No AWS key ID was provided"));
    };
    let Some(aws_key) = params.aws_key else {
        return Err(anyhow!("No AWS secret key was provided"));
    };

    Ok(Params {
        folder,
        schedule,
        aws_region,
        aws_bucket,
        aws_key_id,
        aws_key,
    })
}
