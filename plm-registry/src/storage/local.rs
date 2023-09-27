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

use plm_core::{
    library::store::LibraryStore,
    plm::{package::v1::File, registry::v1::Local},
    utils::fs,
};
use std::{
    fs as _fs,
    str,
};
use tracing::{debug, error, info};

use crate::{error::RegistryError, types::RegistryResult, RegistryStorage};

#[derive(Debug, Clone)]
pub struct LocalStorage {
    pub(crate) storage: Local,
    #[allow(dead_code)]
    pub(crate) registry_path: String,
}

impl RegistryStorage for LocalStorage {
    fn load(&self, library: &str) -> RegistryResult<Vec<File>> {
        let local_storage_path =
            fs::FileSystem::join_paths(self.storage.registry_path.as_str(), library.clone());

        // let protos_dir = fs::FileSystem::join_paths(self.storage.clone().registry_path, library);
        println!("{:?}", local_storage_path);
        let protos =
            LibraryStore::collect(&local_storage_path, local_storage_path.as_path(), &[])
                .map_err(|e| {
                    RegistryError::InvalidConfigSetup(format!(
                        "unable to load protobuf files: {}",
                        e
                    ))
                })?;

        println!("{:?}", protos);

        let mut files = Vec::with_capacity(protos.len());
        for file in protos {
            let file_path = fs::FileSystem::join_paths(local_storage_path.clone(), file.clone());
            let file_name = file_path.clone();
            let content = fs::FileSystem::read_file(file_name.to_str().unwrap()).map_err(|e| {
                RegistryError::InvalidFileContent(format!("failed to load proto file: {:?}", e))
            })?;
            files.push(File {
                name: file.to_string(),
                content: content.into_bytes(),
            })
        }
        Ok(files)
    }

    fn save(&self, library: plm_core::Library) -> RegistryResult<()> {
        let library_path = fs::FileSystem::join_paths(library.name, library.version);
        let local_storage_path =
            fs::FileSystem::join_paths(self.storage.registry_path.as_str(), library_path.clone());
        info!("saving local library: {:?}", local_storage_path);
        fs::FileSystem::create_dir(local_storage_path.clone().to_str().unwrap()).map_err(|e| {
            RegistryError::InvalidConfigSetup(format!(
                "couldnt create directory for new library release {}",
                e
            ))
        })?;
        for pkg in library.packages {
            let pkg_dir = fs::FileSystem::join_paths(
                local_storage_path.clone(),
                pkg.name.as_str().replace('.', "/"),
            );
            let dir_path = pkg_dir.to_str().unwrap();
            debug!("creating package dir: {}", dir_path);
            // fs::FileSystem::create_dir(&dir_path)
            //     .map_err(|e| RegistryError::InvalidFileContent(format!("failed to create directory: {}", dir_path)))?;
            for file in pkg.files {
                let file_path = fs::FileSystem::join_paths(
                    local_storage_path.to_str().unwrap(),
                    file.name.clone(),
                );
                match str::from_utf8(&file.content) {
                    Ok(file_string_content) => {
                        // Create directory if it doesn't exist
                        let file_parent = file_path.parent().unwrap();
                        if let Err(e) = _fs::create_dir_all(file_parent) {
                            error!("Failed to create directory {:?}: {:?}", file_parent, e);
                            return Err(RegistryError::InvalidFileContent(format!(
                                "unable to create directory {:?}",
                                file_parent
                            )));
                        }

                        debug!("saving file to-> {:?}", file_path);
                        fs::FileSystem::write_file(
                            file_path.to_str().unwrap(),
                            file_string_content,
                        )
                        .map_err(|e| {
                            RegistryError::InvalidFileContent(format!(
                                "unable to write {:?}/{}: {:?}",
                                library_path, file.name, e
                            ))
                        })?;
                    }
                    Err(_) => error!("unable to write {:?}/{}", library_path, file.name),
                }
            }
        }

        Ok(())
    }
}
