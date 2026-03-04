use std::{fs, path::PathBuf};

use anyhow::{Result, bail};
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
    pub fn load() -> Result<Self> {
        let base_dirs = directories::BaseDirs::new();
        let config_file = match &base_dirs {
            Some(dirs) => dirs.config_dir().join("speck").join("config.toml"),
            None => bail!("Unable to find user config directory"),
        };

        if !config_file.exists() {
            if let Some(parent) = config_file.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(config_file.as_path(), "")?
        }

        let content = fs::read_to_string(config_file)?;
        let mut config: AppConfig = toml::from_str(&content)?;

        if let Some(dotfiles) = &mut config.dotfiles {
            for dotfile in dotfiles.iter_mut() {
                dotfile.source = shellexpand::full(&dotfile.source)?.into_owned();
                dotfile.destination = shellexpand::full(&dotfile.destination)?.into_owned();
                let source_path = PathBuf::from(&dotfile.source);
                if !source_path.exists() {
                    bail!(
                        "Configured source {} does not exist for {}",
                        source_path.display(),
                        dotfile.name
                    );
                }
            }
        }

        Ok(config)
    }
}
