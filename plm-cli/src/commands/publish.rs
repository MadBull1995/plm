use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use plm_core::{plm::package::v1::File, Compressions, DownloadRequest, FileSystem, Manifest};
use protobuf::{descriptor::FileDescriptorSet, Message};
use tokio::sync::mpsc::Sender;

use crate::{
    helpers::hash_fd_set,
    registry::client::CliRegistryClientBuilder,
    utils::{
        configs::CliConfigs,
        errors::{PlmResult, PlmError},
        prompter::Prompter,
    },
};

pub async fn publish_command(
    manifest: Manifest,
    configs: CliConfigs,
    verbose: bool,
) -> PlmResult<()> {
    let current_dir = &configs.current_dir;

    Prompter::info(format!("publishing: {:<15}", manifest.name).as_str());
    Prompter::task(1, 4, "Collecting '.proto' files");

    // Locate `.proto` files under current library
    let protos_dir = FileSystem::join_paths(current_dir.clone(), manifest.src_dir);
    let files = FileSystem::list_protos(&protos_dir)
        .map_err(|err| PlmError::FileSystemError(err.into()))?;

    // Map the abs file paths to relative for easy protoc compile
    let relative_paths: Result<Vec<String>, io::Error> = files
        .iter()
        .map(|path| FileSystem::to_relative_path(Path::new(path), current_dir))
        .map(|result| result.map(|p| p.to_string_lossy().into_owned()))
        .collect();

    let paths = relative_paths.map_err(|e| {
        Prompter::error(&format!("An error occurred: {}", e));
        PlmError::FileSystemError(e.into())
    })?;

    if verbose {
        for f in paths.iter() {
            Prompter::verbose(f.as_str());
        }
    }

    // Get the vendored protoc bin path
    let protoc = plm_core::protoc::protoc_bin_path()
        .map_err(|err| PlmError::InternalError("Failed to find protoc bin path".to_string()))?;
    std::env::set_var("PROTOC", protoc);

    let lib_dir = FileSystem::current_dir().map_err(|err| PlmError::FileSystemError(err.into()))?;
    let proto_path = FileSystem::to_relative_path(&protos_dir, &lib_dir)
        .map_err(|err| PlmError::FileSystemError(err.into()))?;
    let dot_plm_path = FileSystem::join_paths(lib_dir, ".plm");
    let dot_plm_builds = FileSystem::join_paths(&dot_plm_path, "builds");

    // Compile the proto files using `tonic_build`
    let include_path = plm_core::protoc::include_path()
        .map_err(|err| PlmError::InternalError("Failed to get include path".to_string()))?;

    // Compile the proto files using `tonic_build`
    tonic_build::configure()
        .file_descriptor_set_path(FileSystem::join_paths(&dot_plm_builds, "build.pb"))
        .out_dir(&dot_plm_path)
        .protoc_arg(format!("-I{}", proto_path.to_string_lossy()))
        .build_client(false)
        .build_server(false)
        .build_transport(false)
        .compile(&paths, &[include_path])
        .map_err(|err| PlmError::FileSystemError(err).into())?;

    let build_fd = FileSystem::join_paths(configs.current_dir, ".plm/builds/build.pb");
    let (fd, fd_bytes) = parse_fd_to_protobuf(build_fd)?;
    let packages_to_files = parse_package_files_map(&fd);

    Prompter::task(
        2,
        4,
        format!(
            "Compiled {} files in {} packages",
            paths.len(),
            packages_to_files.keys().len()
        )
        .as_str(),
    );

    if verbose {
        for pkg in packages_to_files.keys() {
            Prompter::verbose(&format!(
                "{}:\n\t - {}",
                pkg,
                packages_to_files.get(pkg).unwrap().join("\n\t - ")
            ))
        }
    }

    Prompter::task(3, 4, "Validating");

    // Generate package metadata
    let parse_packages = |(key, value)| -> plm_core::Package {
        let files_with_content = parse_file_contents(proto_path.clone(), value);
        match files_with_content {
            Ok(f) => plm_core::Package {
                name: key,
                files: f,
                metadata: HashMap::new(),
                ..Default::default()
            },
            Err(e) => {
                Prompter::error(&format!("error on reading proto file: {:?}", e));
                plm_core::Package::default()
            }
        }
    };

    let pkgs = packages_to_files.into_iter().map(parse_packages);
    let release_id = hash_fd_set(fd_bytes);
    let mut lib_md = HashMap::new();
    
    lib_md.insert("checksum".to_string(), release_id.to_string());
    
    if verbose {
        Prompter::verbose(&format!("release: {}", release_id));
    }

    let lib = plm_core::Library {
        name: manifest.name,
        version: manifest.version,
        fd_set: fd_set_to_bytes(&fd),
        metadata: lib_md,
        packages: pkgs.collect(),
    };

    // let mut registry_client_builder = CliRegistryClientBuilder::new()
    //     .build()
    //     .await?;
    // println!("{:?}", registry_client_builder);

    Prompter::task(4, 4, "Upload");
    Ok(())
}

fn parse_file_contents(proto_dir: PathBuf, file_paths: Vec<String>) -> PlmResult<Vec<File>> {
    let mut file_with_contents = Vec::with_capacity(file_paths.len());
    for f in file_paths {
        let content = FileSystem::read_binary_file(&FileSystem::join_paths(proto_dir.clone(), &f))
            .map_err(|err| PlmError::FileSystemError(err.into()))?;
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
            let file_name = &fd.name.clone().unwrap_or_else(|| "".to_string());

            packages_to_files
                .entry(package_name.clone())
                .or_insert_with(Vec::new)
                .push(file_name.clone());
        }
    }
    packages_to_files
}

fn parse_fd_to_protobuf(fd_set_path: PathBuf) -> PlmResult<(FileDescriptorSet, Vec<u8>)> {
    let file = FileSystem::read_binary_file(fd_set_path.as_path())
        .map_err(|err| PlmError::FileSystemError(err))?;

    let fd = protobuf::descriptor::FileDescriptorSet::parse_from_bytes(&file.clone())
        .map_err(|err| PlmError::SerializationError(err.into()))?;

    Ok((fd, file))
}
