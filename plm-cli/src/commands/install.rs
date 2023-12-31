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

use std::path::Path;

use plm_core::{library::store::LibraryStore, plm::library::v1::Dependency, FileSystem, Manifest};

use crate::{
    registry::client::CliRegistryClientBuilder,
    utils::{
        lock::{Library, ProtoLock},
        prompter::Prompter,
    },
    Install,
};

pub async fn install_command(
    install: Install,
    manifest: &mut Manifest,
    manifest_path: &Path,
    proto_lock_path: &Path,
    proto_lock: &mut ProtoLock,
    registry_url: String,
    token: String,
) -> anyhow::Result<()> {
    if let Some(lib_name) = install.name {
        validate_lib_name(&lib_name)?;

        Prompter::task(2, 6, &format!("Installing library -> {}", lib_name));

        // TODO: Fetch Library from Registry
        Prompter::task(3, 6, "Fetching library from registry");

        Prompter::task(4, 6, "Resolving dependencies");
        // Resolve library deps
        let resolved_deps = proto_lock.resolve_dependencies(lib_name.clone())?;

        let mut registry_client_builder = CliRegistryClientBuilder::new();
        registry_client_builder
            .with_addr(registry_url)
            .with_token(token);
        let mut client = registry_client_builder.build().await?;

        let lib = LibraryStore::install(
            Dependency {
                library_id: lib_name.clone(),
                version: "".to_string(),
            },
            &mut client,
        )
        .await?;
        // TODO: Download Dependencies
        Prompter::task(5, 6, "Fetching dependencies");

        // Add library to the proto-lock file
        let installed_lib = Library {
            name: lib_name,
            version: lib.version,
            dependencies: resolved_deps,
        };

        Prompter::task(6, 6, "Updating proto-lock.json file");
        proto_lock.add_library(installed_lib.clone());
        proto_lock.validate()?;
        proto_lock.to_file(proto_lock_path)?;

        manifest
            .dependencies
            .insert(installed_lib.clone().name, installed_lib.version);
        // println!("{:?}", install);
        let path = FileSystem::join_paths(manifest_path.clone(), "proto-package.json");

        FileSystem::write_json(path.to_str().unwrap(), &manifest)
            .map_err(|e| anyhow::anyhow!(e))?;
    } else {
        #[allow(unreachable_code)]
        let mut tree = ":: installing dependencies".to_string();

        let mut registry_client_builder = CliRegistryClientBuilder::new();
        registry_client_builder
            .with_addr(registry_url)
            .with_token(token);
        let mut client = registry_client_builder.build().await?;
        let dependency_count = manifest.dependencies.keys().len();
        for (i, dep) in manifest.clone().dependencies.into_iter().enumerate() {
            let lib = LibraryStore::install(
                Dependency {
                    library_id: dep.0.clone(),
                    version: "".to_string(),
                },
                &mut client,
            )
            .await?;
            let resolved_deps = proto_lock.resolve_dependencies(lib.name.clone())?;

            // Add library to the proto-lock file
            let installed_lib = Library {
                name: lib.name,
                version: lib.version,
                dependencies: resolved_deps,
            };

            Prompter::task(6, 6, "Updating proto-lock.json file");
            proto_lock.add_library(installed_lib.clone());
            proto_lock.validate()?;
            proto_lock.to_file(proto_lock_path)?;

            manifest
                .dependencies
                .insert(installed_lib.clone().name, installed_lib.version.clone());

            let tree_char = if i + 1 == dependency_count {
                '┗'
            } else {
                '┣'
            };

            tree.push_str(&format!(
                "\n   {tree_char} installed {}@{}",
                installed_lib.name, installed_lib.version
            ));
        }
        Prompter::info(&tree);

        let path = FileSystem::join_paths(manifest_path.clone(), "proto-package.json");

        FileSystem::write_json(path.to_str().unwrap(), &manifest)
            .map_err(|e| anyhow::anyhow!(e))?;
    }

    Ok(())
}

fn validate_lib_name(name: &str) -> anyhow::Result<()> {
    if name.starts_with('@') {
        if !name
            .chars()
            .skip(1)
            .all(|c| c.is_ascii_alphabetic() || c == '-' || c == '_' || c == '/')
        {
            return Err(anyhow::anyhow!("Library name must consist of ASCII alphabetic characters, dash, lower dash, or forward slash"));
        }
    } else if !name
        .chars()
        .all(|c| c.is_ascii_alphabetic() || c == '-' || c == '_')
    {
        return Err(anyhow::anyhow!(
            "Library name must consist of ASCII alphabetic characters, dash, or lower dash"
        ));
    }
    Ok(())
}
