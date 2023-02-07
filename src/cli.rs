use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the folder to backup
    ///
    /// If not specified, defaults to "/dockerbox"
    #[arg(value_hint = clap::ValueHint::DirPath)]
    pub folder: Option<PathBuf>,

    /// Specify a cron expression to run the backup periodically
    ///
    /// If not specified, the backup will only run once
    #[arg(short, long, value_name = "CRON")]
    pub schedule: Option<String>,
}
