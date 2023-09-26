#[macro_use]
extern crate diesel_migrations;
extern crate lazy_static;

pub mod storage {

    pub trait RegistryStorage: Send + Sync {
        fn save(&self, library: plm_core::Library ) -> RegistryResult<()>;
        fn load(&self, library: &str) -> RegistryResult<Vec<File>>;
    }

    mod builder;
    pub mod local;
    pub mod s3;
    use std::collections::HashMap;

    pub use builder::StorageBuilder;
    use plm_core::plm::package::v1::File;
    use tonic::async_trait;

    use crate::types::RegistryResult;
}

pub mod data {
    mod builder;
    pub mod models;
    pub mod psql;
    pub mod schema;
    pub use builder::DataBuilder;
}

pub mod api {
    mod server;
    pub mod service;
    pub use server::{RegistryServer, RegistryServerBuilder};
}

pub mod utils {
    pub mod auth;
    pub mod config;
    pub mod error;
    pub mod tracing;
    pub mod types;
}

pub use crate::{api::*, data::*, storage::*, utils::*};
