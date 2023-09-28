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


// extern crate protoc_bin_vendored::{include_path, protoc_bin_path};

pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    // protoc_bin_vendored
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "../protos/plm/package/v1/manifest.proto",
                "../protos/plm/package/v1/lock.proto",
                "../protos/plm/registry/v1/server.proto",
                "../protos/plm/registry/v1/config.proto",
                "../protos/plm/registry/v1/storage.proto",
                "../protos/plm/registry/v1/registry.proto",
                "../protos/plm/library/v1/library.proto",
                "../protos/plm/user/v1/user.proto",
                "../protos/plm/organization/v1/organization.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}
