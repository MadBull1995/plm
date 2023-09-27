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

#[macro_use]
extern crate diesel_migrations;
extern crate lazy_static;

pub mod storage {

    pub trait RegistryStorage: Send + Sync {
        fn save(&self, library: plm_core::Library) -> RegistryResult<()>;
        fn load(&self, library: &str) -> RegistryResult<Vec<File>>;
    }

    mod builder;
    pub mod local;
    pub mod s3;

    pub use builder::StorageBuilder;
    use plm_core::plm::package::v1::File;

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
