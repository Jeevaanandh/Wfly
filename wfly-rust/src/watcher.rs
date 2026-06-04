use notify::{Event, RecursiveMode, Result, Watcher};
use std::env;
use std::time::{Duration, Instant};
use std::{path::Path, sync::mpsc};

use crate::starter::start_server;

pub async fn watch(key: &str) -> notify::Result<()> {
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(&current_dir), RecursiveMode::Recursive);

    println!("Directory is being watched: {:?}", current_dir);

    start_server(&current_dir, key, 0, 1).await;

    let mut last_run = Instant::now();
    for res in rx {
        match res {
            Ok(event) => {
                let event_instant = Instant::now();

                let elapsed = last_run.elapsed();

                if elapsed > Duration::from_secs(2) {
                    let file = event.paths[0].to_str().unwrap().to_string();

                    if file.contains("/target/") {
                        continue;
                    }

                    if !file.ends_with(".java") && !file.ends_with(".xml") {
                        continue;
                    }

                    println!("Changes Detected in: {:?}", event.paths[0]);

                    start_server(&current_dir, key, 0, 0).await;
                    last_run = event_instant;
                }
            }

            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
