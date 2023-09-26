use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use plm_core::{
    library::store::LibraryStore, plm::package::v1::File, Compressions, DownloadRequest,
    FileSystem, Manifest, PublishRequest,
};
use protobuf::{descriptor::FileDescriptorSet, Message};
use tokio::sync::mpsc::Sender;
use tonic::Status;

use crate::{
    registry::client::CliRegistryClientBuilder,
    utils::{
        configs::CliConfigs,
        errors::{PlmError, PlmResult},
        prompter::Prompter,
    },
};

pub async fn publish_command(
    manifest: Manifest,
    configs: CliConfigs,
    token: String
) -> Result<()> {
    let current_dir = &configs.current_dir;

    Prompter::info(format!("publishing: {:<15}", manifest.name).as_str());
    Prompter::task(1, 4, "Collecting '.proto' files");

    let lib = LibraryStore::release(current_dir, manifest).await?;

    let mut registry_client_builder = CliRegistryClientBuilder::new();
    registry_client_builder
        .with_addr(configs.registry)
        .with_token(token);
    let mut client = registry_client_builder.build().await?;
    let mut publish = PublishRequest::default();
    publish.lib = Some(lib);
    client.publish(publish).await?;

    Prompter::task(4, 4, "Upload");
    Ok(())
}
