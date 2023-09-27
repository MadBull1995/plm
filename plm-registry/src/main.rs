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

// Std libs
use ::tracing::debug;
use plm_core::Config;
use std::env;
// Own libs
use plm_registry::{
    config::ConfigBuilder,
    psql::{establish_connection, initialize_schema},
    storage::RegistryStorage,
    tracing::setup_tracing,
    types::RegistryResult,
    DataBuilder,
    RegistryServer,
    RegistryServerBuilder,
    StorageBuilder,
};

#[tokio::main]
async fn main() -> RegistryResult<()> {
    // Starting Protobuf Package Manager registry
    let mut configs = ConfigBuilder::new();
    let config = setup_configs(&mut configs)?;
    setup_tracing(&(config.clone().server.unwrap().log_level as u8));
    let mut storage = StorageBuilder::new();
    #[allow(unused_assignments)]
    let mut store_path = None;
    match config.storage.unwrap().storage_backend {
        None => store_path = Some("plm_registry".to_string()),
        Some(storgae_backend) => match storgae_backend {
            plm_core::plm::registry::v1::storage::StorageBackend::Local(local) => {
                let path = local.registry_path.clone();
                store_path = Some(path)
            }
            plm_core::plm::registry::v1::storage::StorageBackend::S3(_) => {
                todo!()
            }
        },
    }
    let storage = setup_storage(&mut storage, &store_path.unwrap())?;

    let mut db = DataBuilder::new();
    setup_db(&mut db).await?;
    dbg!(db);

    let server_builder = RegistryServerBuilder::new(storage);
    let tmp_server_cfg = config.server.unwrap();
    let addr = format!("{}:{}", tmp_server_cfg.host, tmp_server_cfg.port);
    let server = setup_server(&mut server_builder.clone(), addr)?;

    server.run().await;

    Ok(())
}

fn setup_configs(cfg_builder: &mut ConfigBuilder) -> RegistryResult<Config> {
    // Get custom config path if any
    let args: Vec<String> = env::args().collect();
    // Check if a config path is provided
    if args.len() > 1 {
        let config_path = &args[1];
        debug!("loading custom config file: {}", config_path);
        match ConfigBuilder::from_json_file(config_path) {
            // Or use from_toml_file
            Ok(loaded_config) => *cfg_builder = loaded_config,
            Err(e) => eprintln!("Failed to load config from {}: {}", config_path, e),
        }
    }

    let cfg = cfg_builder.clone().build();

    // Set the psql host
    let data = cfg.data.clone().unwrap();
    let psql_host = format!(
        "postgres://{}:{}@{}/{}",
        data.username, data.password, data.host, "registrydb"
    );

    std::env::set_var("DATABASE_URL", psql_host);

    Ok(cfg)
}

fn setup_storage(
    storage_builder: &mut StorageBuilder,
    store_path: &str,
) -> RegistryResult<Box<dyn RegistryStorage + Sync + Send>> {
    // TODO: Validate registry path for local storage / ping S3
    debug!("setting up storage");

    let storage = storage_builder.with_store_path(store_path);
    Ok(Box::new(storage.clone().build()))
}

async fn setup_db(_db_builder: &mut DataBuilder) -> RegistryResult<()> {
    debug!("setting up database");
    let mut pool = establish_connection();
    initialize_schema(&mut pool);

    Ok(())
}

fn setup_server(
    server_builder: &mut RegistryServerBuilder,
    addr: String,
) -> RegistryResult<RegistryServer> {
    Ok(server_builder.with_addr(addr).clone().build())
}

// use diesel::prelude::*;

// table! {
//     users (user_id) {
//         user_id -> Int4,
//         username -> Varchar,
//         email -> Varchar,
//         password_hash -> Varchar,
//         created_at -> Timestamptz,
//         updated_at -> Timestamptz,
//     }
// }

// table! {
//     organizations (org_id) {
//         org_id -> Int4,
//         name -> Varchar,
//         created_at -> Timestamptz,
//         updated_at -> Timestamptz,
//     }
// }

// table! {
//     user_organizations (user_id, org_id) {
//         user_id -> Int4,
//         org_id -> Int4,
//         role -> Int4
//     }
// }

// table! {
//     libraries (lib_id) {
//         lib_id -> Int4,
//         name -> Varchar,
//         version -> Varchar,
//         org_id -> Nullable<Int4>,
//         public -> Bool,
//         created_at -> Timestamptz,
//         updated_at -> Timestamptz,
//     }
// }
