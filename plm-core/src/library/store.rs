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

use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use protobuf::{descriptor::FileDescriptorSet, Message};
use tokio::fs;
use tracing::{debug, info};

use crate::{
    manifest::MANIFEST_FILE,
    plm::{
        library::v1::{Dependency, Library},
        package::v1::File,
    },
    FileSystem, Manifest,
};

/// IO abstraction layer over local `plm` library store
#[derive(Debug)]
pub struct LibraryStore;

impl LibraryStore {
    /// Path to the proto directory
    pub const PROTO_MODULES_PATH: &str = "proto_modules";

    /// Creates the expected directory structure for `plm`
    pub async fn create() -> Result<()> {
        let create = |dir: &'static str| async move {
            let path = Path::new(dir.clone());
            fs::create_dir_all(dir).await.with_context(|| {
                format!(
                    "Failed to create dependency folder {}",
                    path.to_string_lossy()
                )
            })
        };
        create(Self::PROTO_MODULES_PATH).await?;

        Ok(())
    }

    /// Clears all packages from the file system
    pub async fn clear() -> Result<()> {
        fs::remove_dir_all(Self::PROTO_MODULES_PATH)
            .await
            .with_context(|| "Failed to uninstall dependencies")
    }

    /// Unpacks a library into a local directory
    pub async fn unpack(library: &Library) -> Result<()> {
        let lib_dir = Path::new(Self::PROTO_MODULES_PATH).join(library.name.as_str());
        // .join(library.version.as_str());

        // fs::remove_dir_all(&lib_dir).await.ok();

        fs::create_dir_all(&lib_dir)
            .await
            .with_context(|| "Failed to install dependencies")?;

        // library

        for pkg in library.clone().packages {
            if !pkg.name.eq("") {
                let pkg_dir = Path::new(&lib_dir).join(pkg.name.as_str());
                fs::remove_dir_all(&pkg_dir).await.ok();
                debug!("clearing package directory: {:?}", pkg_dir);
            }
            for file in pkg.files {
                let file_path = Path::new(&lib_dir).join(file.name);
                fs::create_dir_all(file_path.parent().unwrap()).await?;
                tracing::trace!("writing file: {:?}", file_path);
                fs::write(file_path, file.content).await?;
            }
        }

        debug!(
            "unpacked {}@{} into {}",
            library.name,
            library.version,
            lib_dir.display()
        );

        Ok(())
    }

    /// Installs a package and all of its dependency into the local filesystem
    pub async fn install<R: crate::registry::Registry>(
        dependency: Dependency,
        registry: &mut R,
    ) -> Result<()> {
        let library = registry.download(dependency).await?;

        debug!("downloaded: {}:{}", library.name, library.version);

        Self::unpack(&library).await?;

        let tree = format!(":: installed {}@{}", library.name, library.version);

        let Manifest { dependencies, .. } = Self::resolve(&Dependency {
            library_id: library.name,
            version: library.version,
        })
        .await?;

        let _dependency_count = dependencies.len();

        for (index, dependency) in dependencies.into_iter().enumerate() {
            println!("i: {} - {:?}", index, dependency);
            // if let Ok(manifest) = Self::resolve(&dependency.package).await {
            // let existing = manifest.package.wrap_err(eyre::eyre!(
            //     "Found installed manifest for {} but it is malformed",
            //     dependency.package,
            // ))?;

            // eyre::ensure!(
            //     dependency.manifest.version.matches(&existing.version),
            //     "A dependency of your project requires {}@{} which collides with {}@{} required by {}",
            //     existing.name,
            //     existing.version,
            //     dependency.package,
            //     dependency.manifest.version,
            //     package.manifest.name,
            // );
            // }

            // let dependency = registry.download(dependency).await?;

            // Self::unpack(&dependency).await?;

            // let tree_char = if index + 1 == dependency_count {
            //     '┗'
            // } else {
            //     '┣'
            // };

            // tree.push_str(&format!(
            //     "\n   {tree_char} installed {}@{}",
            //     dependency.manifest.name, dependency.manifest.version
            // ));
        }

        info!("{tree}");

        Ok(())
    }

    /// Uninstalls a package from the local file system
    // pub async fn uninstall(package: &PackageId) -> eyre::Result<()> {
    //     let pkg_dir = Path::new(Self::PROTO_VENDOR_PATH).join(package.as_str());

    //     fs::remove_dir_all(&pkg_dir)
    //         .await
    //         .wrap_err_with(|| format!("Failed to uninstall {package}"))
    // }

    /// Resolves a package in the local file system
    pub async fn resolve(lib: &Dependency) -> Result<Manifest> {
        let manifest = Self::locate(lib).join(MANIFEST_FILE);
        let manifest = FileSystem::read_manifest(manifest.to_str().unwrap())
            .with_context(|| format!("Failed to locate local manifest for package: {:?}", lib))?;
        Ok(manifest)
    }

    /// Packages a release from the local file system state
    pub async fn release(current_dir: &Path, manifest: Manifest) -> Result<Library> {
        for dependency in manifest.dependencies.iter() {
            let _resolved = Self::resolve(&Dependency {
                library_id: dependency.0.to_string(),
                version: dependency.1.to_string(),
            })
            .await
            .with_context(|| "Failed to resolve dependency locally")?;
        }

        let lib_path = fs::canonicalize(&manifest.src_dir)
            .await
            .with_context(|| format!("failed to canonicalize the src_dir: {}", manifest.src_dir))?;
        info!("{:?}", lib_path);
        // let mut archive = tar::Builder::new(Vec::new());

        // let manifest = toml::to_string_pretty(&RawManifest::from(manifest))
        //     .wrap_err("Failed to encode release manifest")?
        //     .into_bytes();

        // let mut header = tar::Header::new_gnu();
        // header.set_size(manifest.len().try_into().wrap_err("Failed to pack tar")?);
        // header.set_mode(0o444);
        // archive
        //     .append_data(&mut header, MANIFEST_FILE, Cursor::new(manifest))
        //     .wrap_err("Failed to add manifest to release")?;
        // let protos_dir = FileSystem::join_paths(current_dir, manifest.src_dir);
        let paths = Self::collect(&lib_path, current_dir, &manifest.exclude)?;
        // println!("{:?}", protos_dir);
        // Get the vendored protoc bin path
        let protoc = crate::protoc::protoc_bin_path()
            .with_context(|| "Failed to find protoc bin path".to_string())?;
        std::env::set_var("PROTOC", protoc);

        let rel_proto_path = FileSystem::to_relative_path(&lib_path, current_dir)
            .with_context(|| "failed to get relative proto src dir".to_string())?;

        let dot_plm_path = FileSystem::join_paths(current_dir, ".plm");
        let dot_plm_builds = FileSystem::join_paths(&dot_plm_path, "builds");

        // Compile the proto files using `tonic_build`
        let include_path = crate::protoc::include_path()
            .with_context(|| "Failed to get include path".to_string())?;
        let mut proto_path = rel_proto_path.to_str().unwrap();
        if proto_path.is_empty() {
            proto_path = "."
        }
        // info!("{:?} -> {}", rel_proto_path, rel_proto_path.to_str().unwrap() != lib_path.to_str().unwrap() );
        if proto_path != "." && !FileSystem::dir_exists(proto_path) {
            // Prompter::error();
            Err(anyhow!(
                "must have a valid 'src_dir' value pointing to a root .proto files directory"
            ))
        } else {
            // Compile the proto files using `tonic_build`
            tonic_build::configure()
                .file_descriptor_set_path(FileSystem::join_paths(dot_plm_builds, "build.pb"))
                .out_dir(&dot_plm_path)
                .protoc_arg(format!("-I{}", proto_path))
                .build_client(false)
                .build_server(false)
                .build_transport(false)
                .compile(&paths, &[include_path])
                .with_context(|| "failed to run protoc successfully")?;

            let build_fd = FileSystem::join_paths(current_dir, ".plm/builds/build.pb");
            let (fd, fd_bytes) = parse_fd_to_protobuf(build_fd)?;
            let packages_to_files = parse_package_files_map(&fd);

            // Generate package metadata
            let parse_packages = |(key, value)| -> crate::Package {
                let files_with_content = parse_file_contents(rel_proto_path.clone(), value);
                match files_with_content {
                    Ok(f) => crate::Package {
                        name: key,
                        files: f,
                        metadata: HashMap::new(),
                        ..Default::default()
                    },
                    Err(_e) => {
                        // Prompter::error(&format!("error on reading proto file: {:?}", e));
                        crate::Package::default()
                    }
                }
            };

            let pkgs = packages_to_files.into_iter().map(parse_packages);
            let release_id = crate::utils::hash_fd_set(fd_bytes);
            let mut lib_md = HashMap::new();

            lib_md.insert("checksum".to_string(), release_id.to_string());

            let lib = crate::Library {
                name: manifest.name,
                version: manifest.version,
                fd_set: fd_set_to_bytes(&fd),
                metadata: lib_md,
                packages: pkgs.collect(),
            };

            Ok(lib)
        }
    }

    /// Directory for the vendored installation of a package
    pub fn locate(library: &Dependency) -> PathBuf {
        PathBuf::from(Self::PROTO_MODULES_PATH)
            .join(library.library_id.clone())
            .join(library.version.clone())
    }

    // Collect .proto files in a given path whilst excluding vendored ones
    pub fn collect(protos_dir: &Path, lib_path: &Path, exclude: &[String]) -> Result<Vec<String>> {
        // Locate `.proto` files under current library
        let files = FileSystem::list_protos(protos_dir)
            .with_context(|| "failed to collect library .proto files".to_string())?;

        // Map the abs file paths to relative for easy protoc compile
        let relative_paths: Result<Vec<String>, io::Error> = files
            .iter()
            .filter(|path| {
                // Convert to relative path
                let rel_path = FileSystem::to_relative_path(Path::new(path), lib_path)
                    .unwrap_or_else(|_| path.into());

                // Check if the path should be excluded
                !exclude.iter().any(|ex| rel_path.starts_with(ex))
            })
            .map(|path| FileSystem::to_relative_path(Path::new(path), lib_path))
            .map(|result| result.map(|p| p.to_string_lossy().into_owned()))
            .collect();

        let mut paths =
            relative_paths.with_context(|| "failed to process relative paths".to_string())?;

        paths.sort(); // to ensure determinism

        Ok(paths)
    }
}

fn parse_file_contents(proto_dir: PathBuf, file_paths: Vec<String>) -> Result<Vec<File>> {
    let mut file_with_contents = Vec::with_capacity(file_paths.len());
    for f in file_paths {
        let content = FileSystem::read_binary_file(&FileSystem::join_paths(proto_dir.clone(), &f))
            .with_context(|| "failed to read proto file".to_string())?;
        file_with_contents.push(File { name: f, content });
    }

    Ok(file_with_contents)
}

fn fd_set_to_bytes(fd_set: &FileDescriptorSet) -> Vec<u8> {
    fd_set.write_to_bytes().unwrap()
}

fn parse_package_files_map(fd_set: &FileDescriptorSet) -> HashMap<String, Vec<String>> {
    let mut packages_to_files = HashMap::new();

    for fd in &fd_set.file {
        if let Some(package_name) = &fd.package {
            let file_name = &fd.name.clone().unwrap_or_default();

            packages_to_files
                .entry(package_name.clone())
                .or_insert_with(Vec::new)
                .push(file_name.clone());
        }
    }
    packages_to_files
}

fn parse_fd_to_protobuf(fd_set_path: PathBuf) -> Result<(FileDescriptorSet, Vec<u8>)> {
    let file = FileSystem::read_binary_file(fd_set_path.as_path())
        .with_context(|| "failed to read file descriptor set".to_string())?;

    let fd = protobuf::descriptor::FileDescriptorSet::parse_from_bytes(&file.clone())
        .with_context(|| "failed to parse file descriptor set".to_string())?;

    Ok((fd, file))
}
