// Std libs
use plm_core::Config;
use std::{collections::HashSet, env};
// Own libs
use plm_registry::{
    api,
    config::ConfigBuilder,
    data,
    types::RegistryResult,
    utils::{auth, error, tracing},
    DataBuilder, ServerBuilder, StorageBuilder,
};

#[tokio::main]
async fn main() -> RegistryResult<()> {
    // Starting Protobuf Package Manager registry
    let mut configs = ConfigBuilder::new();
    let cfg = setup_configs(&mut configs)?;
    dbg!(cfg);

    let mut storage = StorageBuilder::new();
    setup_storage(&mut storage)?;
    dbg!(storage);

    let mut db = DataBuilder::new();
    setup_db(&mut db)?;
    dbg!(db);

    let mut server = ServerBuilder::new();
    setup_server(&mut server)?;
    dbg!(server);

    // server.serve().await?;

    Ok(())
}

fn setup_configs(cfg_builder: &mut ConfigBuilder) -> RegistryResult<Config> {
    // Get custom config path if any
    let args: Vec<String> = env::args().collect();
    // Check if a config path is provided
    if args.len() > 1 {
        let config_path = &args[1];
        match ConfigBuilder::from_json_file(config_path) {
            // Or use from_toml_file
            Ok(loaded_config) => *cfg_builder = loaded_config,
            Err(e) => eprintln!("Failed to load config from {}: {}", config_path, e),
        }
    }

    let cfg = cfg_builder.clone().build();

    Ok(cfg)
}

fn setup_storage(storage_builder: &mut StorageBuilder) -> RegistryResult<()> {
    // TODO: Validate registry path for local storage / ping S3

    Ok(())
}

fn setup_db(db_builder: &mut DataBuilder) -> RegistryResult<()> {
    // TODO: Validate psql connection

    Ok(())
}

fn setup_server(server_builder: &mut ServerBuilder) -> RegistryResult<()> {
    Ok(())
}
