// Copyright 2023 PLM Authors
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

use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::{error::RegistryError, types::RegistryResult};

const REGISTRY_HOST: &str = "127.0.0.1";
const REGISTRY_PORT: u32 = 7575;
const REGISTRY_PATH: &str = "proto_registry";
const _REGISTRY_LOG_LEVEL: &str = "info";

const DB_HOST: &str = "localhost:5432";
const DB_USER: &str = "plm_admin";
const DB_PASS: &str = "plm_admin";

pub type StorageSetup = plm_core::plm::registry::v1::storage::StorageBackend;
pub type ServerSetup = plm_core::plm::registry::v1::Server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBuilder {
    server: ServerSetup,
    storage: StorageSetup,
    data: plm_core::Data,
    // Add more options here as needed
}
impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigBuilder {
    // Create a new builder with some default values
    pub fn new() -> Self {
        Self {
            server: default_server_setup(),
            // log_level: REGISTRY_LOG_LEVEL.to_string(),
            data: default_db_setup(),
            storage: default_storage_setup(),
        }
    }

    // Setter methods for each configuration option
    pub fn host(&mut self, host: &str) -> &mut Self {
        self.server.host = host.to_string();
        self
    }

    pub fn port(&mut self, port: u32) -> &mut Self {
        self.server.port = port;
        self
    }

    pub fn data(&mut self, data_setup: plm_core::Data) -> &mut Self {
        self.data = data_setup;
        self
    }

    // pub fn log_level(&mut self, log_level: &str) -> &mut Self {
    //     self.log_level = log_level.to_string();
    //     self
    // }

    pub fn from_json_file(filepath: &str) -> RegistryResult<Self> {
        let mut file = File::open(filepath)
            .map_err(|err| RegistryError::InvalidConfigSetup(err.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| RegistryError::InvalidConfigSetup(err.to_string()))?;
        let config = serde_json::from_str(&contents)
            .map_err(|err| RegistryError::InvalidConfigSetup(err.to_string()))?;
        Ok(config)
    }

    pub fn build(self) -> plm_core::Config {
        plm_core::Config {
            data: Some(self.data),
            server: Some(self.server),
            storage: Some(plm_core::Storage {
                storage_backend: Some(self.storage),
            }),
        }
    }
}

fn default_server_setup() -> ServerSetup {
    ServerSetup {
        host: REGISTRY_HOST.to_string(),
        port: REGISTRY_PORT,
        log_level: 0,
    }
}

fn default_storage_setup() -> StorageSetup {
    StorageSetup::Local(plm_core::Local {
        registry_path: default_local_registry_path(),
    })
}

fn default_local_registry_path() -> String {
    let curr_dir = plm_core::FileSystem::current_dir()
        .map_err(|err| RegistryError::InvalidConfigSetup(err.to_string()))
        .unwrap();
    String::from(
        plm_core::FileSystem::join_paths(curr_dir, REGISTRY_PATH)
            .as_path()
            .to_str()
            .unwrap(),
    )
}

fn default_db_setup() -> plm_core::Data {
    plm_core::Data {
        host: DB_HOST.to_string(),
        username: DB_USER.to_string(),
        password: DB_PASS.to_string(),
    }
}
