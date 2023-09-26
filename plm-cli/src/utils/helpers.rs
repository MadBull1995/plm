use std::path::PathBuf;

use plm_core::{FileSystem as fs, Manifest};
use serde::Serialize;

use super::{
    errors::{PlmError, PlmResult},
    prompter::Prompter,
};

pub fn get_global_plmrc_path() -> PathBuf {
    fs::join_paths(fs::get_home_directory().unwrap(), ".plmrc")
}

pub fn get_manifest_from_file() -> PlmResult<plm_core::Manifest> {
    let manifest_path = fs::join_paths(fs::current_dir().unwrap(), "proto-package.json");
    Prompter::verbose(format!("reading manifest from: {:?}", manifest_path).as_str());
    let mfst = fs::read_manifest(manifest_path.clone().as_path().to_str().unwrap())
        .map_err(|err| PlmError::InternalError("Failed to parse manifest from file".to_string()))?;

    Ok(mfst)
}
