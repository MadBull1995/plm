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

use std::path::PathBuf;

use futures::Stream;
use indicatif::ProgressBar;
use plm_core::FileSystem as fs;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::codegen::Pin as TonicPin;

use super::{
    errors::{PlmError, PlmResult},
    prompter::Prompter,
};

pub fn get_global_plmrc_path() -> PathBuf {
    fs::join_paths(fs::get_home_directory().unwrap(), ".plmrc")
}

pub fn get_manifest_from_file() -> PlmResult<plm_core::Manifest> {
    let manifest_path = fs::join_paths(fs::current_dir().unwrap(), "proto-package.json");
    Prompter::verbose(format!("Reading manifest from: {:?}", manifest_path).as_str());
    let mfst =
        fs::read_manifest(manifest_path.clone().as_path().to_str().unwrap()).map_err(|_err| {
            PlmError::InternalError("Failed to parse manifest from file".to_string())
        })?;

    Ok(mfst)
}

pub struct ProgressStream<S> {
    pub(crate) inner: S,
    pub(crate) pb: ProgressBar,
}

impl<S: Stream + Unpin> Stream for ProgressStream<S> {
    type Item = S::Item;

    fn poll_next(self: TonicPin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut(); // Get a mutable reference to `self`
        let next = Pin::new(&mut this.inner).poll_next(cx); // Re-borrowing the Pin
        if next.is_ready() {
            this.pb.inc(1);
        }
        next
    }
}

pub fn bytes_to_human_readable(mut bytes: usize) -> String {
    let mut count = 0;
    let units = ["B", "KB", "MB", "GB", "TB"];

    while bytes >= 1024 && count < units.len() - 1 {
        bytes /= 1024;
        count += 1;
    }

    format!("{} {}", bytes, units[count])
}
