mod cli;
mod config;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, CommandRunner, Commands};

fn main() -> Result<()> {
    let command_runner = CommandRunner::new();
    let cli = Cli::parse();
    let msg = match &cli.command {
        Commands::Link { files } => command_runner?.link(files)?,
        Commands::Unlink { files } => command_runner?.unlink(files)?,
        Commands::Clone { repos, rm } => command_runner?.clone(repos, *rm)?,
        Commands::List { dotfiles, repos } => command_runner?.list(*dotfiles, *repos)?,
    };
    println!("{}", msg.trim());
    Ok(())
}
