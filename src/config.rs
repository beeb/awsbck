use std::{env, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::{command, Parser};
use job_scheduler_ng::Schedule;

use crate::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the folder to backup
    ///
    /// If not specified, defaults to "/dockerbox"
    #[arg(value_hint = clap::ValueHint::DirPath)]
    folder: Option<PathBuf>,

    /// Specify a cron expression to run the backup periodically
    ///
    /// If not specified, the backup will only run once
    #[arg(short, long, value_name = "CRON")]
    schedule: Option<String>,

    /// A Dropbox access token
    #[arg(short = 't', long = "token", value_name = "KEY")]
    dropbox_token: Option<String>,
}

/// Runtime parameters, parsed and ready to be used
pub struct Params {
    /// Which folder to backup
    pub folder: PathBuf,
    /// An optional parsed cron expression
    pub schedule: Option<Schedule>,
    /// A Dropbox access token
    pub dropbox_token: String,
}

/// Parse the command-line arguments and environment variables into runtime params
pub fn parse_config() -> Result<Params> {
    let mut params = Cli::parse();

    params.folder = params
        .folder
        .or_else(|| env::var("DOCKERBOX_FOLDER").ok().map(PathBuf::from))
        .or_else(|| Some(PathBuf::from("/dockerbox")));

    params.schedule = params
        .schedule
        .or_else(|| env::var("DOCKERBOX_SCHEDULE").ok());

    params.dropbox_token = params
        .dropbox_token
        .or_else(|| env::var("DOCKERBOX_TOKEN").ok());

    let folder = params.folder.or_panic(); // Ok to unwrap due to default value
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

    let Some(dropbox_token) = params.dropbox_token else {
        return Err(anyhow!("No Dropbox token was provided"));
    };

    Ok(Params {
        folder,
        schedule,
        dropbox_token,
    })
}
