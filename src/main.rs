mod cli;
mod config;

use clap::Parser;
use cli::{Cli, CommandRunner, Commands};

fn main() {
    let command_runner = CommandRunner::new();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Link { files } => {
            command_runner.link(files);
        }
        Commands::Unlink { files } => {
            command_runner.unlink(files);
        }
        Commands::Clone { repos, rm } => {
            command_runner.clone(repos, *rm);
        }
    }
}
