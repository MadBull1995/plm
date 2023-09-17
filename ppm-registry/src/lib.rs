pub mod storage {

    #[async_trait]
    pub trait RegistryStorage {
        async fn save(&self) -> RegistryResult<()>;
        async fn load(&self, package: &str) -> RegistryResult<()>;
    }

    pub mod local;
    pub mod s3;
    mod builder;
    pub use builder::StorageBuilder;
    use tonic::async_trait;

    use crate::types::RegistryResult;
}

pub mod data {
    pub mod psql;
    mod builder;

    pub use builder::DataBuilder;
}

pub mod api {
    mod server;
    pub mod service;

    pub use server::ServerBuilder;
}

pub mod utils {
    pub mod auth;
    pub mod tracing;
    pub mod config;
    pub mod types;
    pub mod error;
}

pub use crate::{
    utils::*,
    api::*,
    data::*,
    storage::*
};