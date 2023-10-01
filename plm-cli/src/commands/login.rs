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

use anyhow::Context;
use plm_core::LoginRequest;
use tonic::Code;

use crate::{
    registry::client::CliRegistryClientBuilder,
    utils::{configs::CliConfigs, prompter::Prompter},
};

pub async fn login_command(
    configs: &mut CliConfigs,
    username: &str,
    token: &str,
    registry_url: String,
) -> anyhow::Result<()> {
    Prompter::info("Processing login to registry");
    let mut client_builder = CliRegistryClientBuilder::new();
    client_builder.with_addr(registry_url.clone());
    let mut client = client_builder.build().await?;
    let login = LoginRequest {
        username: username.to_string(),
        token: token.to_string(),
    };

    let login_response = client.login(login.clone()).await;
    match login_response {
        Err(e) => match e.code() {
            Code::NotFound => {
                Prompter::warning(e.message());
                client
                    .create_user(login)
                    .await
                    .with_context(|| format!("failed to login to registry: {:?}", registry_url))?;
            }
            Code::Unauthenticated => {
                Prompter::error(e.message());
            }
            _ => Prompter::error(e.message()),
        },
        Ok(jwt) => {
            Prompter::info(&format!(
                "login successfully to registry: {}\n{:?}",
                registry_url, jwt
            ));
            configs.token = Some(jwt.token);
            configs.write_plmrc_file()?;
            // TODO: Save verified username + pass to .plmrc global file
        }
    }
    Ok(())
}
