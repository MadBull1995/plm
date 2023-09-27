// Copyright 2023 Sylk Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{collections::HashMap, path::PathBuf};

use plm_core::FileSystem;

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

impl Default for CliConfigs {
    fn default() -> Self {
        Self::new()
    }
}

impl CliConfigs {
    pub fn new() -> Self {
        Self {
            current_dir: FileSystem::current_dir().unwrap(),
            username: None,
            password: None,
            registry: DEFAULT_REGISTRY.to_string(),
            token: None,
        }
    }

    pub fn to_json(&self) {
        let d = DotPlmRC {
            registry: self.registry.clone(),
            token: self.token.clone(),
            username: self.username.clone(),
        };

        println!("{}", serde_json::to_string_pretty(&d).unwrap());
    }

    pub fn write_plmrc_file(&self) -> PlmResult<()> {
        let content = format!(
            "registry={}\nusername={}\ntoken={}",
            self.registry,
            self.username.clone().unwrap_or_default(),
            self.token.clone().unwrap_or_default(),
        );
        FileSystem::write_file(
            FileSystem::join_paths(FileSystem::get_home_directory().unwrap(), ".plmrc")
                .to_str()
                .unwrap(),
            &content,
        )
        .map_err(PlmError::FileSystemError)
    }

    pub fn load_plmrc_files(&mut self) -> PlmResult<()> {
        #[allow(unused_assignments)]
        let mut overrides: Option<HashMap<String, String>> = None;

        let global = FileSystem::parse_plmrc_file(true);

        match global {
            Err(_err) => {
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
                        Prompter::warning(&format!(
                            "key: {} is not supported on .plmrc config file",
                            k
                        ))
                    }
                }
            }
        }

        let local = FileSystem::parse_plmrc_file(false);

        match local {
            Err(_) => {}
            Ok(plmrc) => {
                Prompter::verbose(
                    "found local project .plmrc file, going to override global ~/.plmrc",
                );
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
