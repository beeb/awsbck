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

mod backup;
mod config;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var(
            "RUST_LOG",
            "awsbck=info,aws_config=warn,aws_credential_types=warn,tracing=warn",
        );
    };
    env_logger::init();

    let params = Arc::new(parse_config().await?);

    info!("Will backup \"{}\"", params.folder.to_string_lossy());

    match params.interval {
        Some(interval) => {
            let task = task::spawn(async move {
                let shared_params = Arc::clone(&params);
                let mut interval = time::interval(Duration::from_secs(interval));
                loop {
                    interval.tick().await;
                    match backup(&shared_params).await {
                        Ok(_) => {
                            info!("Backup succeeded");
                        }
                        Err(e) => {
                            error!("Backup error: {e:#}");
                        }
                    }
                }
            });
            task.await?;
        }
        None => {
            backup(&params)
                .await
                .with_context(|| anyhow!("Backup error"))?;
            info!("Backup succeeded");
        }
    }
    Ok(())
}
