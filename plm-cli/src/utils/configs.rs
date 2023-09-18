use std::{path::PathBuf, collections::HashMap};

use plm_core::FileSystem;

use super::{errors::{PlmResult, PlmError}, prompter::Prompter};

pub struct CliConfigBuilder {}

pub const DEFAULT_REGISTRY: &str = "http://[::1]:7575";

#[derive(Debug, Clone)]
pub struct CliConfigs {
    pub current_dir: PathBuf,
    pub username: Option<String>,
    pub password: Option<String>,
    pub registry: String,
}

impl CliConfigs {
    pub fn new() -> Self {
        Self {
            current_dir: FileSystem::current_dir().unwrap(),
            username: None,
            password: None,
            registry: DEFAULT_REGISTRY.to_string(),
        }
    }

    pub fn load_plmrc_files(&mut self) -> PlmResult<()> {
        let mut overrides: Option<HashMap<String,String>> = None;
        
        let global = FileSystem::parse_plmrc_file(true);

        match global {
            Err(err) => {
                Prompter::warning("failed to load global configs $HOME/.plmrc");
                overrides = Some(HashMap::new());
            }
            Ok(g) => {
                overrides = Some(g);
            },
        }
    
        let local = FileSystem::parse_plmrc_file(false);

        match local {
            Err(_) => {},
            Ok(plmrc) => {
                dbg!(plmrc);
            }
        }

        match overrides {
            None => {

            },
            Some(plmrc) => {
                for k in plmrc.keys() {
                    if k == "username" {
                        self.username = Some(plmrc.get(k).unwrap().to_string());
                    } else if k == "password" {
                        self.password = Some(plmrc.get(k).unwrap().to_string());
                    }
                }
            }
        }
        Ok(())
    }
}