mod db;
mod starter;
mod watcher;

use clap::{Parser, Subcommand};
use db::{add_path, db_init, get_keys};
use libc::{SIGINT, signal};
use starter::start_server;
use watcher::watch;

use starter::handle_sigint;

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
    },

    GetKeys,

    //This is to just start the standalone.sh
    Run {
        #[arg(long)]
        key: String,

        #[arg(long)]
        offset: Option<u64>,
    },
}

#[tokio::main]
async fn main() {
    unsafe {
        signal(SIGINT, handle_sigint as usize);
    }

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

        Cmd::Set { key, path } => {
            match add_path(&pool, &path, &key).await {
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
            if let Some(offset) = offset {
                start_server("", &key, offset, 1).await;
            } else {
                start_server("", &key, 0, 1).await;
            }

            loop {}
        }
    }
}
