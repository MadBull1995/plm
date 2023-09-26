use anyhow::Context;
use plm_core::LoginRequest;
use tonic::{Status, Code};

use crate::{
    registry::client::{CliRegistryClient, CliRegistryClientBuilder},
    utils::{errors::{PlmResult, PlmError}, prompter::Prompter, configs::CliConfigs},
};

pub async fn login_command(
    configs: &mut CliConfigs,
    username: &str,
    token: &str,
    registry_url: String,
) -> anyhow::Result<()> {
    Prompter::info("processing login to registry");
    let mut client_builder = CliRegistryClientBuilder::new();
    client_builder.with_addr(registry_url.clone());
    let mut client = client_builder.build().await?;
    // println!("{:?}", client);
    let mut login = LoginRequest::default();

    login.username = username.to_string();
    login.token = token.to_string();
    let login_response = client.login(login.clone()).await;
    match login_response {
        Err(e) => {
            match e.code() {
                Code::NotFound => {
                    Prompter::warning(e.message());
                    client
                        .create_user(login)
                        .await
                        .with_context(|| format!("failed to login to registry: {:?}", registry_url))?;
                },
                Code::Unauthenticated => {
                    Prompter::error(e.message());
                },
                _ =>  Prompter::error(e.message())
            }
        }
        Ok(jwt) => {
            Prompter::info(&format!("login successfully to registry: {}\n{:?}", registry_url, jwt));
            configs.token = Some(jwt.token);
            configs.write_plmrc_file()?;
            // TODO: Save verified username + pass to .plmrc global file
        }
    }
    Ok(())
}
