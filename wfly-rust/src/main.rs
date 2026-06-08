mod db;
mod starter;
mod watcher;

use std::ptr::null;

use clap::{Parser, Subcommand};
use ctrlc;
use db::{add_path, db_init, get_keys, get_offset, update_entry};
use starter::start_server;
use watcher::watch;

use starter::CHILD_PID;

#[derive(Parser)]
#[command(name = "docsearch", about = "A hotreload tool for Wildfly servers")]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Start {
        #[arg(long)]
        key: String,
    },

    Set {
        #[arg(long)]
        key: String,

        #[arg(long)]
        path: String,

        #[arg(long)]
        offset: Option<i32>,
    },

    GetKeys,

    //This is to just start the standalone.sh
    Run {
        #[arg(long)]
        key: String,

        #[arg(long)]
        offset: Option<u64>,
    },

    UpdateKey {
        #[arg(long)]
        key: String,

        #[arg(long)]
        path: Option<String>,

        #[arg(long)]
        offset: Option<i32>,
    },

    RunAll,
}

#[tokio::main]
async fn main() {
    ctrlc::set_handler(move || {
        unsafe {
            for &pid in CHILD_PID.lock().unwrap().iter() {
                libc::kill(-pid, libc::SIGTERM);
            }
        }

        std::process::exit(0);
    })
    .unwrap();

    let args = Args::parse();

    let pool = match db_init().await {
        Ok(p) => p,

        Err(e) => {
            println!("Error in creating the pool\n\n");

            println!("ERROR: {:?}", e);

            return;
        }
    };

    match args.command {
        Cmd::Start { key } => {
            match watch(&key).await {
                Ok(_) => {}

                Err(e) => {
                    println!("Error in starting the wildfly server\n\n");

                    println!("ERROR: {:?}", e);
                }
            };
        }

        Cmd::Set { key, path, offset } => {
            //ie. if offset is mentiond ---- use it, otherwise = 0
            let offset = match offset {
                Some(o) => o,

                None => 0,
            };

            match add_path(&pool, &path, &key, offset).await {
                Ok(_) => {
                    println!("The Wildfly Home path was added successfully");
                }

                Err(e) => {
                    println!("Error in ading the Wildfly Home path\n\n");

                    println!("ERROR: {:?}", e);
                }
            };
        }

        Cmd::GetKeys => {
            let keys = match get_keys(&pool).await {
                Ok(k) => k,

                Err(e) => {
                    println!("Error in getting the keys from the DB");
                    println!("ERROR: {:?}", e);
                    return;
                }
            };

            for key in keys {
                println!("{:?}", key);
            }
        }

        Cmd::Run { key, offset } => {
            let offset = match offset {
                Some(o) => o,

                None => get_offset(&pool, &key).await as u64,
            };

            start_server("", &key, offset, 1).await;

            loop {}
        }

        Cmd::RunAll => {
            let keys = match get_keys(&pool).await {
                Ok(k) => k,

                Err(e) => {
                    println!("Error in getting the keys from the DB");
                    println!("ERROR: {:?}", e);

                    return;
                }
            };

            for k in keys {
                let offset = get_offset(&pool, &k).await as u64;
                start_server("", &k, offset, 1).await;
            }

            loop {}
        }

        Cmd::UpdateKey { key, path, offset } => {
            let path: String = match path {
                Some(p) => p,

                None => "".to_string(),
            };

            let offset = match offset {
                Some(o) => o,

                None => -1,
            };

            if path.is_empty() && offset == -1 {
                println!("Provide a path or the offset to update");
                return;
            }

            let keys = get_keys(&pool).await.unwrap();

            if !keys.contains(&key) {
                println!("Could not find the key: {:?}", key);
                return;
            }

            match update_entry(&pool, &key, &path, offset).await {
                Ok(_) => {
                    println!("Successfully updated!!!");
                }

                Err(e) => {
                    println!("Error in updating the the Key");
                    println!("ERROR: {:?}", e);
                }
            };
        }
    }
}
