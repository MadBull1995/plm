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

use std::path::PathBuf;

use plm_core::FileSystem as fs;

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
        .map_err(|_err| PlmError::InternalError("Failed to parse manifest from file".to_string()))?;

    Ok(mfst)
}
