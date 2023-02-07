//! Utility to backup a docker volume (or any folder) to AWS S3, once or periodically.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use dotenvy::dotenv;
use tokio_cron_scheduler::{Job, JobScheduler};

use backup::backup;
use config::parse_config;

mod backup;
mod config;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let params = Arc::new(parse_config().await?);

    println!("Will backup '{}'", params.folder.to_string_lossy());

    match params.schedule.clone() {
        Some(schedule) => {
            let sched = JobScheduler::new().await?;
            let job = Job::new_async(schedule, move |_, _| {
                let shared_params = Arc::clone(&params);
                Box::pin(async move {
                    match backup(&shared_params).await {
                        Ok(_) => println!("Backup succeeded"),
                        Err(e) => eprintln!("Backup error: {e:#?}"),
                    }
                })
            })?;
            sched.add(job).await?;
        }
        None => {
            backup(&params)
                .await
                .with_context(|| anyhow!("Backup error"))?;
            println!("Backup succeeded");
        }
    }

    Ok(())
}
