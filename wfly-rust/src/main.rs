mod starter;
mod watcher;

use clap::{Parser, Subcommand};
use watcher::watch;

use crate::starter::start_server;

#[derive(Parser)]
#[command(name = "docsearch", about = "A hotreload tool for Wildfly servers")]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Start,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Cmd::Start => {
            watch();
        }
    }
}
