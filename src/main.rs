//! Utility to backup a folder to AWS S3, once or periodically.
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{env, sync::Arc};

use anyhow::{Context, Result};
use chrono::Utc;
use dotenvy::dotenv;
use log::{error, info};
use tokio::{
    task,
    time::{self, Instant},
};

use backup::backup;
use config::parse_config;

use crate::prelude::*;

mod aws;
mod backup;
mod config;
mod prelude;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    dotenv().ok(); // load .env file if present

    // set default logging level. we ignore info logs from aws
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            "awsbck=info,aws_config=warn,aws_credential_types=warn,tracing=warn",
        );
    };
    env_logger::init();

    // parse config from env and/or cli
    let params = Arc::new(parse_config().await?);

    // check if we run the backup once, or periodically forever
    if let Some(schedule) = &params.schedule {
        info!(
            "Will backup \"{}\" on cron schedule: \"{}\"",
            params.folder.to_string_lossy(),
            schedule.to_string()
        );
        // spawn a routine that will run the backup periodically
        let task = task::spawn({
            let params = Arc::clone(&params);
            async move {
                loop {
                    // we checked that the schedule exists above, so we can unwrap it
                    let Some(deadline) = params.schedule.as_ref().or_panic().upcoming(Utc).next()
                    else {
                        error!("Could not get next execution time for cron schedule");
                        return;
                    };
                    info!("Next backup scheduled for {}", deadline.to_rfc2822());
                    // tokio's sleep_until` expect an `Instant` and not a Utc::DateTime, let's convert.
                    // first we get the duration between now and the deadline
                    let Ok(wait_time) = (deadline - Utc::now()).to_std() else {
                        error!("Could not convert duration to std Duration");
                        return;
                    };
                    // then we add it to the current instant
                    let Some(deadline) = Instant::now().checked_add(wait_time) else {
                        error!("Could not convert deadline to tokio Instant");
                        return;
                    };
                    // and finally we sleep until the next cron execution time
                    time::sleep_until(deadline).await;
                    // run the backup
                    match backup(&params).await {
                        Ok(()) => {
                            info!("Backup succeeded");
                        }
                        Err(e) => {
                            // we handle errors here to keep the loop running
                            error!("Backup error: {e:#}");
                        }
                    }
                }
            }
        });
        let ctrl_c = tokio::spawn(async move {
            tokio::signal::ctrl_c().await.or_panic();
        });
        // loop forever, unless ctrl-c is called
        tokio::select! {
            _ = ctrl_c => {
                info!("Ctrl-C received, exiting");
            }
            _ = task => {
                info!("Backup task exited, exiting");
            }
        }
    } else {
        // run backup only once, immediately
        info!("Backing up \"{}\" once", params.folder.to_string_lossy());
        backup(&params).await.context("Backup error")?;
        info!("Backup succeeded");
    }
    Ok(())
}
