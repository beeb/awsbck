//! Utility to backup a folder to AWS S3, once or periodically.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{env, sync::Arc, time::Duration};

use anyhow::{anyhow, Context, Result};
use dotenvy::dotenv;
use log::*;
use tokio::{task, time};

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
    match params.interval {
        Some(interval) => {
            info!(
                "Will backup \"{}\" every {interval} seconds",
                params.folder.to_string_lossy()
            );
            // spawn a routine that will run the backup periodically
            let task = task::spawn(async move {
                let shared_params = Arc::clone(&params);
                let mut interval = time::interval(Duration::from_secs(interval));
                loop {
                    interval.tick().await; // the first tick completes immediately, triggering the backup
                    match backup(&shared_params).await {
                        Ok(_) => {
                            info!("Backup succeeded");
                        }
                        Err(e) => {
                            // we handle errors here to keep the loop running
                            error!("Backup error: {e:#}");
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
        }
        None => {
            // run backup only once, immediately
            info!("Backuping \"{}\" once", params.folder.to_string_lossy());
            backup(&params)
                .await
                .with_context(|| anyhow!("Backup error"))?;
            info!("Backup succeeded");
        }
    }
    Ok(())
}
