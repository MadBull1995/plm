use std::path::PathBuf;

use plm_core::{FileSystem as fs, Manifest};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    errors::{PlmResult, PlmError},
    prompter::Prompter,
};

pub fn get_global_plmrc_path() -> PathBuf {
    fs::join_paths(fs::get_home_directory().unwrap(), ".plmrc")
}

pub fn get_manifest_from_file(verbose: &bool) -> PlmResult<plm_core::Manifest> {
    let manifest_path = fs::join_paths(fs::current_dir().unwrap(), "proto-package.json");
    if *verbose {
        Prompter::verbose(format!("reading manifest from: {:?}", manifest_path).as_str());
    }
    let mfst = fs::read_manifest(manifest_path.clone().as_path().to_str().unwrap())
        .map_err(|err| PlmError::InternalError("Failed to parse manifest from file".to_string()))?;

    Ok(mfst)
}

pub fn hash_fd_set(fd_set_bytes: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&fd_set_bytes);
    let hash = hasher.finalize();
    let hex_string = hex::encode(hash);

    hex_string
}
