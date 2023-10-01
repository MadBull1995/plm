// Copyright 2023 PLM Authors
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

use anyhow::Result;

use crate::{
    registry::client::CliRegistryClientBuilder,
    utils::{configs::CliConfigs, prompter::Prompter},
};
use plm_core::{library::store::LibraryStore, Manifest, PublishRequest};

pub async fn publish_command(
    manifest: Manifest,
    configs: CliConfigs,
    token: String,
    preserve_imports: bool,
) -> Result<()> {
    let current_dir = &configs.current_dir;
    Prompter::info(format!("Publishing: {:<15}", manifest.name).as_str());
    if preserve_imports {
        Prompter::warning(
            "Passed --preserve-imports flag, will resolve import files without plm prefixing",
        )
    }
    Prompter::task(1, 3, "Collecting .proto files and compilling...");
    let lib = LibraryStore::release(current_dir, manifest, preserve_imports).await?;

    let mut registry_client_builder = CliRegistryClientBuilder::new();
    registry_client_builder
        .with_addr(configs.registry.clone())
        .with_token(token);
    let mut client = registry_client_builder.build().await?;
    let publish = PublishRequest {
        lib: Some(lib.clone()),
    };
    Prompter::task(2, 3, "Creating new release for library");
    client.publish(publish).await?;
    Prompter::task(3, 3, "Uploading .proto files to registry");
    client.upload(lib).await?;
    Prompter::success(&format!("Library published: {}", configs.registry));
    Ok(())
}
