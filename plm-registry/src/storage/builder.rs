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

use plm_core::Local;

use crate::local::LocalStorage;

#[derive(Debug, Clone)]
pub struct StorageBuilder {
    store_path: String,
}

impl Default for StorageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageBuilder {
    pub fn new() -> Self {
        Self {
            store_path: "plm_registry".to_string(),
        }
    }

    pub fn with_store_path(&mut self, storage_path: &str) -> &mut Self {
        self.store_path = storage_path.to_string();
        self
    }

    pub fn build(self) -> LocalStorage {
        LocalStorage {
            storage: Local {
                registry_path: self.store_path.clone(),
            },
            registry_path: self.store_path.clone(),
        }
    }
}
