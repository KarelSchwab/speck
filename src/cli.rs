use std::{fs, os, path::PathBuf};

use anyhow::{Ok, Result, bail};
use clap::{Parser, Subcommand};
use git2::Repository;

use crate::config::AppConfig;

#[derive(Parser)]
#[command(name = "dotfiles")]
#[command(version = "1.0")]
#[command(about = "Dotfile manager written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Link {
        files: Option<Vec<String>>,
    },
    Unlink {
        files: Option<Vec<String>>,
    },
    Clone {
        #[arg(long, num_args= 1..)]
        repos: Option<Vec<String>>,
        #[arg(long)]
        rm: bool,
    },
}

pub struct CommandRunner {
    app_config: AppConfig,
}

impl CommandRunner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            app_config: AppConfig::load()?,
        })
    }

    pub fn link(self, files: &Option<Vec<String>>) -> Result<String> {
        let mut out = String::from("Successfully linked ");
        if let Some(mut dotfiles) = self.app_config.dotfiles {
            if let Some(names) = files {
                dotfiles.retain(|dotfile| names.contains(&dotfile.name));
            }
            for dotfile in dotfiles {
                let destination_path = PathBuf::from(dotfile.destination);
                if destination_path.exists() {
                    fs::remove_dir_all(destination_path.as_path())?;
                }

                let source_path = PathBuf::from(dotfile.source);
                os::unix::fs::symlink(source_path.as_path(), destination_path.as_path())?;
                out.push_str(&format!("{} ", dotfile.name));
            }
        }
        Ok(out)
    }

    pub fn unlink(self, files: &Option<Vec<String>>) -> Result<String> {
        let mut out = String::from("Successfully unlinked ");
        if let Some(mut dotfiles) = self.app_config.dotfiles {
            if let Some(names) = files {
                dotfiles.retain(|dotfile| names.contains(&dotfile.name));
            }
            for dotfile in dotfiles {
                let destination_path = PathBuf::from(dotfile.destination);
                if destination_path.exists() {
                    fs::remove_dir_all(destination_path.as_path())?;
                    out.push_str(&format!("{} ", dotfile.name));
                }
            }
        }
        Ok(out)
    }

    pub fn clone(self, repos: &Option<Vec<String>>, rm: bool) -> Result<String> {
        let mut out = String::from("Successfully cloned ");
        if let Some(mut git_repos) = self.app_config.repos {
            if let Some(names) = repos {
                git_repos.retain(|dotfile| names.contains(&dotfile.name));
            }
            for repo in git_repos {
                let destination_path = PathBuf::from(repo.destination);
                if destination_path.exists() {
                    if rm {
                        fs::remove_dir_all(destination_path.as_path())?;
                    } else {
                        bail!(
                            "Cannot clone {} to {}. The destination already exists. Re-run with --rm to remove it first",
                            repo.name,
                            destination_path.display()
                        );
                    }
                }

                Repository::clone(&repo.url, &destination_path)?;
                out.push_str(&format!("{} ", repo.name));
            }
        }
        Ok(out)
    }
}
