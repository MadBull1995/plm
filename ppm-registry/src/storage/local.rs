use ppm_core::ppm::registry::v1::Local;
use tonic::async_trait;

use crate::{RegistryStorage, types::RegistryResult};

#[derive(Debug, Clone)]
pub struct LocalStorage {
    storage: Local
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