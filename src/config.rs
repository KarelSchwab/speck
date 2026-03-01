use std::{fs, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Dotfile {
    pub name: String,
    pub source: String,
    pub destination: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub destination: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct AppConfig {
    pub dotfiles: Option<Vec<Dotfile>>,
    pub repos: Option<Vec<Repo>>,
}

impl AppConfig {
    pub fn load() -> Self {
        let base_dirs = directories::BaseDirs::new();
        let config_file = match &base_dirs {
            Some(dirs) => dirs.config_dir().join("dotfiles").join("config.toml"),
            None => panic!("Unable to find user config directory"),
        };

        if !config_file.exists() {
            // TODO: Create config file
            panic!("Config file does not exist")
        }

        let content = match fs::read_to_string(config_file) {
            Ok(content) => content,
            Err(_) => panic!("Error reading config file"),
        };

        let mut config = match toml::from_str(&content) {
            Ok(config) => config,
            Err(_) => {
                println!("Error deserialising config file");
                AppConfig::default()
            }
        };

        if let Some(dotfiles) = &mut config.dotfiles {
            dotfiles.iter_mut().for_each(|dotfile| {
                dotfile.source = match shellexpand::full(&dotfile.source) {
                    Ok(s) => s.into_owned(),
                    Err(e) => panic!("Unable to expand vars: {}", e.cause),
                };
                dotfile.destination = match shellexpand::full(&dotfile.destination) {
                    Ok(s) => s.into_owned(),
                    Err(e) => panic!("Unable to expand vars: {}", e.cause),
                };

                let source_path = PathBuf::from(&dotfile.source);
                if !source_path.exists() {
                    panic!(
                        "Configured source {} does not exist for {}",
                        source_path.display(),
                        dotfile.name
                    );
                }
            });
        }

        config
    }
}
