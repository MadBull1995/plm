use plm_core::Local;

use crate::local::LocalStorage;

#[derive(Debug, Clone)]
pub struct StorageBuilder {
    store_path: String,
}

impl StorageBuilder {
    pub fn new() -> Self {
        Self {
            store_path: "plm_registry".to_string()
        }
    }

    pub fn with_store_path(&mut self, storage_path: &str) -> &mut Self {
        self.store_path = storage_path.to_string();
        self
    }

    pub fn build(self) -> LocalStorage {
        LocalStorage {
            storage: Local {
                registry_path: self.store_path.clone()
                
            },
            registry_path: self.store_path.clone(),
        }
    }
}


