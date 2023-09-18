pub mod storage {

    #[async_trait]
    pub trait RegistryStorage {
        async fn save(&self) -> RegistryResult<()>;
        async fn load(&self, package: &str) -> RegistryResult<()>;
    }

    mod builder;
    pub mod local;
    pub mod s3;
    pub use builder::StorageBuilder;
    use tonic::async_trait;

    use crate::types::RegistryResult;
}

pub mod data {
    mod builder;
    pub mod psql;

    pub use builder::DataBuilder;
}

pub mod api {
    mod server;
    pub mod service;

    pub use server::ServerBuilder;
}

pub mod utils {
    pub mod auth;
    pub mod config;
    pub mod error;
    pub mod tracing;
    pub mod types;
}

pub use crate::{api::*, data::*, storage::*, utils::*};
