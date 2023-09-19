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
    DataBuilder, StorageBuilder, RegistryServerBuilder, RegistryServer,
};

#[tokio::main]
async fn main() -> RegistryResult<()> {
    // Starting Protobuf Package Manager registry
    let mut configs = ConfigBuilder::new();
    let config = setup_configs(&mut configs)?;

    let mut storage = StorageBuilder::new();
    setup_storage(&mut storage)?;
    dbg!(storage);

    let mut db = DataBuilder::new();
    setup_db(&mut db)?;
    dbg!(db);

    let mut server_builder = RegistryServerBuilder::new();
    let tmp_server_cfg = config.server.unwrap();
    let addr = format!("{}:{}", tmp_server_cfg.host, tmp_server_cfg.port);
    let server = setup_server(&mut server_builder.clone(), addr)?;
    dbg!(server.clone());

    server.run().await;

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

fn setup_server(server_builder:&mut RegistryServerBuilder, addr: String) -> RegistryResult<RegistryServer> {
    Ok(server_builder
        .with_addr(addr)
        .build())
}
