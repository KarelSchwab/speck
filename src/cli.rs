use std::{fs, os, path::PathBuf};

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
    pub fn new() -> Self {
        Self {
            app_config: AppConfig::load(),
        }
    }

    pub fn link(self, files: &Option<Vec<String>>) {
        if let Some(mut dotfiles) = self.app_config.dotfiles {
            if let Some(names) = files {
                dotfiles.retain(|dotfile| names.contains(&dotfile.name));
            }
            for dotfile in dotfiles {
                let destination_path = PathBuf::from(dotfile.destination);
                if destination_path.exists() {
                    match fs::remove_dir_all(destination_path.as_path()) {
                        Ok(_) => println!("Successfully removed {}", destination_path.display()),
                        Err(_) => panic!("Cannot remove {}", destination_path.display()),
                    }
                }

                let source_path = PathBuf::from(dotfile.source);
                match os::unix::fs::symlink(source_path.as_path(), destination_path.as_path()) {
                    Ok(_) => println!("Successfully stowed {}", dotfile.name),
                    Err(e) => panic!("Unable to stow {}: {}", dotfile.name, e),
                }
            }
        }
    }

    pub fn unlink(self, files: &Option<Vec<String>>) {
        if let Some(mut dotfiles) = self.app_config.dotfiles {
            if let Some(names) = files {
                dotfiles.retain(|dotfile| names.contains(&dotfile.name));
            }
            for dotfile in dotfiles {
                let destination_path = PathBuf::from(dotfile.destination);
                if destination_path.exists() {
                    match fs::remove_dir_all(destination_path.as_path()) {
                        Ok(_) => println!("Successfully unstowed {}", destination_path.display()),
                        Err(_) => panic!("Cannot unstow {}", destination_path.display()),
                    }
                }
            }
        }
    }

    pub fn clone(self, repos: &Option<Vec<String>>, rm: bool) {
        if let Some(mut git_repos) = self.app_config.repos {
            if let Some(names) = repos {
                git_repos.retain(|dotfile| names.contains(&dotfile.name));
            }
            for repo in git_repos {
                let destination_path = PathBuf::from(repo.destination);
                if destination_path.exists() {
                    if rm {
                        match fs::remove_dir_all(destination_path.as_path()) {
                            Ok(_) => {
                                println!("Successfully removed {}", destination_path.display())
                            }
                            Err(_) => panic!("Cannot remove {}", destination_path.display()),
                        }
                    } else {
                        panic!(
                            "Cannot clone {} to {}. The destination already exists. Re-run with --rm to remove it first",
                            repo.name,
                            destination_path.display()
                        );
                    }
                }

                // Clone repo
                match Repository::clone(&repo.url, &destination_path) {
                    Ok(_) => println!(
                        "Successfully cloned {} to {}",
                        repo.name,
                        destination_path.display()
                    ),
                    Err(e) => panic!("Unable to clone {}: {}", repo.name, e),
                }
            }
        }
    }
}
