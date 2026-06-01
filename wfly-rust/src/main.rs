mod db;
mod starter;
mod watcher;

use clap::{Parser, Subcommand};
use db::{add_path, db_init, get_path};
use libc::{SIGINT, signal};
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
    Start,

    Set { path: String },
}

#[tokio::main]
async fn main() {
    unsafe {
        signal(SIGINT, handle_sigint as usize);
    }

    let args = Args::parse();

    match args.command {
        Cmd::Start => {
            match watch().await {
                Ok(_) => {}

                Err(e) => {
                    println!("Error in starting the wildfly server\n\n");

                    println!("ERROR: {:?}", e);
                }
            };
        }

        Cmd::Set { path } => {
            let pool = match db_init().await {
                Ok(p) => p,

                Err(e) => {
                    println!("Error in creating the pool\n\n");

                    println!("ERROR: {:?}", e);

                    return;
                }
            };

            match add_path(&pool, &path).await {
                Ok(_) => {
                    println!("The Wildfly Home path was added successfully");
                }

                Err(e) => {
                    println!("Error in ading the Wildfly Home path\n\n");

                    println!("ERROR: {:?}", e);
                }
            };
        }
    }
}
