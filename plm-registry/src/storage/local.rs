use plm_core::plm::registry::v1::Local;
use tonic::async_trait;

use crate::{types::RegistryResult, RegistryStorage};

#[derive(Debug, Clone)]
pub struct LocalStorage {
    storage: Local,
}

#[async_trait]
impl RegistryStorage for LocalStorage {
    async fn load(&self, package_name: &str) -> RegistryResult<()> {
        Ok(())
    }

    async fn save(&self) -> RegistryResult<()> {
        Ok(())
    }
}
