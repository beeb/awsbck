//! Utility to backup a docker volume (or any folder) to Dropbox, once or periodically.
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use dotenvy::dotenv;
use job_scheduler_ng::{Job, JobScheduler};

use backup::backup;
use config::parse_config;

mod backup;
mod config;
mod prelude;

fn main() -> Result<()> {
    dotenv().ok();

    let params = parse_config()?;

    println!("Will backup '{}'", params.folder.to_string_lossy());

    match params.schedule {
        Some(schedule) => {
            let mut sched = JobScheduler::new();
            sched.add(Job::new(schedule, || {
                match backup(&params.folder, &params.dropbox_token) {
                    Ok(_) => println!("Backup succeeded"),
                    Err(e) => eprintln!("Backup error: {e:#?}"),
                }
            }));
            loop {
                sched.tick();
                std::thread::sleep(Duration::from_millis(500));
            }
        }
        None => {
            backup(&params.folder, &params.dropbox_token)
                .with_context(|| anyhow!("Backup error"))?;
            println!("Backup succeeded");
        }
    }

    Ok(())
}
