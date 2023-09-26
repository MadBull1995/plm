use std::{collections::HashMap, env, path::PathBuf};

use plm_core::{FileSystem, Config};

use super::{
    errors::{PlmError, PlmResult},
    prompter::Prompter,
};

pub struct CliConfigBuilder {}

pub const DEFAULT_REGISTRY: &str = "http://[::1]:7575";

#[derive(Debug, Clone)]
pub struct CliConfigs {
    pub current_dir: PathBuf,
    pub username: Option<String>,
    pub password: Option<String>,
    pub registry: String,
    pub token: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DotPlmRC {
    pub registry: String,
    pub username: Option<String>,
    pub token: Option<String>,
}

impl CliConfigs {
    pub fn new() -> Self {
        Self {
            current_dir: FileSystem::current_dir().unwrap(),
            username: None,
            password: None,
            registry: DEFAULT_REGISTRY.to_string(),
            token: None
        }
    }

    pub fn to_json(&self) {

        let d = DotPlmRC {
            registry: self.registry.clone(),
            token: self.token.clone(),
            username:self.username.clone()
        };

        println!("{}", serde_json::to_string_pretty(&d).unwrap());
    }

    pub fn write_plmrc_file(&self) -> PlmResult<()> {
        let content = format!(
            "registry={}\nusername={}\ntoken={}",
            self.registry,
            self.username.clone().unwrap_or_else(|| "".to_string()),
            self.token.clone().unwrap_or_else(|| "".to_string()),
        );
        FileSystem::write_file(
            FileSystem::join_paths(FileSystem::get_home_directory().unwrap(), ".plmrc")
                .to_str()
                .unwrap(),
            &content,
        )
        .map_err(|err| PlmError::FileSystemError(err))
    }

    pub fn load_plmrc_files(&mut self) -> PlmResult<()> {
        let mut overrides: Option<HashMap<String, String>> = None;

        let global = FileSystem::parse_plmrc_file(true);

        match global {
            Err(err) => {
                Prompter::warning("failed to load global configs $HOME/.plmrc, creating one.");
                self.write_plmrc_file()?;
                overrides = Some(HashMap::new());
            }
            Ok(g) => {
                overrides = Some(g);
            }
        }

        match overrides {
            None => {}
            Some(plmrc) => {
                Prompter::verbose("reading global configs: ~/.plmrc");
                for k in plmrc.keys() {
                    if k == "username" {
                        self.username = Some(plmrc.get(k).unwrap().to_string());
                    } else if k == "password" {
                        self.password = Some(plmrc.get(k).unwrap().to_string());
                    } else if k == "registry" {
                        self.registry = plmrc.get(k).unwrap().to_string();
                    } else if k == "token" {
                        self.token = Some(plmrc.get(k).unwrap().to_string());
                    } else {
                        Prompter::warning(&format!("key: {} is not supported on .plmrc config file", k))
                    }
                }
            }
        }

        let local = FileSystem::parse_plmrc_file(false);

        match local {
            Err(_) => {}
            Ok(plmrc) => {
                Prompter::verbose("found local project .plmrc file, going to override global ~/.plmrc");
                for k in plmrc.keys() {
                    if k == "username" {
                        self.username = Some(plmrc.get(k).unwrap().to_string());
                    } else if k == "password" {
                        self.password = Some(plmrc.get(k).unwrap().to_string());
                    } else if k == "registry" {
                        self.registry = plmrc.get(k).unwrap().to_string();
                    }
                }
            }
        }

        Ok(())
    }
}
