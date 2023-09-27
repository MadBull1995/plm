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

use std::io;
pub type PlmResult<T> = anyhow::Result<T, PlmError>;

#[derive(thiserror::Error, Debug)]
pub enum PlmError {
    #[error("Some internal error occurred: {0}")]
    InternalError(String),

    #[error("Error on file system operation: {0:?}")]
    FileSystemError(io::Error),

    #[error("Error on serialization operation: {0:?}")]
    SerializationError(io::Error),
}
