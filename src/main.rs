//! Utility to backup a folder to AWS S3, once or periodically.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use dotenvy::dotenv;
use tokio::sync::mpsc;
use tokio_cron_scheduler::{Job, JobScheduler};

use backup::backup;
use config::parse_config;

mod backup;
mod config;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let (tx, mut rx) = mpsc::unbounded_channel();

    let params = Arc::new(parse_config().await?);

    println!("Will backup '{}'", params.folder.to_string_lossy());

    match params.schedule.clone() {
        Some(schedule) => {
            let mut sched = JobScheduler::new().await?;
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
            sched.shutdown_on_ctrl_c();
            sched.set_shutdown_handler(Box::new(move || {
                let tx = tx.clone();
                Box::pin(async move {
                    let _ = tx.send(true);
                    println!("Shut down done");
                })
            }));
            sched.start().await?;
            rx.recv().await;
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
