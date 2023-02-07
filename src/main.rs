use std::{path::PathBuf, time::Duration};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use dotenvy::dotenv;
use job_scheduler_ng::{Job, JobScheduler};

use backup::backup;
use cli::Cli;

mod backup;
mod cli;

fn main() -> Result<()> {
    dotenv().ok();

    let params = Cli::parse();

    let folder = params.folder.unwrap_or_else(|| PathBuf::from("/dockerbox"));
    if !folder.is_dir() {
        return Err(anyhow!(
            "The folder '{}' does not exist",
            folder.to_string_lossy()
        ));
    }
    println!("Will backup '{}'", folder.canonicalize()?.to_string_lossy());

    match params.schedule {
        Some(schedule) => {
            let mut sched = JobScheduler::new();
            sched.add(Job::new(schedule.parse()?, || match backup() {
                Ok(_) => println!("Backup succeeded"),
                Err(e) => eprintln!("Backup error: {e:#?}"),
            }));
            loop {
                sched.tick();
                std::thread::sleep(Duration::from_millis(500));
            }
        }
        None => {
            backup().with_context(|| anyhow!("Backup error"))?;
            println!("Backup succeeded");
        }
    }

    Ok(())
}
