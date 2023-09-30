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

use std::process::exit;

// use anyhow::{Context, Ok};
use plm_core::{
    registry_service_client, user_service_client, DownloadRequest, LoginRequest, LoginResponse,
    ProtobufOrGz, PublishRequest,
};
use tonic::{
    async_trait,
    metadata::{Ascii, MetadataValue},
    service::{interceptor::InterceptedService, Interceptor},
    transport::{Channel, Endpoint},
    Status,
};

use crate::utils::{
    errors::{PlmError, PlmResult},
    prompter::Prompter,
};

// You can also use the `Interceptor` trait to create an interceptor type
// that is easy to name
pub struct AuthInterceptor {
    pub token: MetadataValue<Ascii>,
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.token.clone());
        Ok(request)
    }
}

#[derive(Debug)]
pub struct CliRegistryClientBuilder {
    addr: Option<String>,
    token: Option<String>,
}

impl Default for CliRegistryClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CliRegistryClientBuilder {
    pub fn new() -> Self {
        Self {
            addr: Some("http://localhost:8080".to_string()),
            token: None,
        }
    }

    pub fn with_addr(&mut self, addr: String) -> &mut Self {
        let socket_addr = addr.parse().unwrap();
        self.addr = Some(socket_addr);

        self
    }

    pub fn with_token(&mut self, token: String) -> &mut Self {
        self.token = Some(token);

        self
    }

    pub async fn build(self) -> PlmResult<CliRegistryClient> {
        let addr = self.addr.unwrap().to_owned();
        let channel = Endpoint::from_shared(addr)
            .unwrap()
            .connect()
            .await
            .map_err(|err| {
                PlmError::InternalError(format!("failed to build gRPC channel: {:?}", err))
            })?;
        let token: MetadataValue<_> =
            format!("Bearer {}", self.token.clone().unwrap_or("".to_string()))
                .parse()
                .map_err(|_| {
                    PlmError::InternalError(
                        "failed to attach token to outgoing request".to_string(),
                    )
                })?;

        let reg = registry_service_client::RegistryServiceClient::with_interceptor(
            channel.clone(),
            AuthInterceptor { token },
        )
        .max_decoding_message_size(100 * 1024 * 1024) // 100 MB
        .max_encoding_message_size(100 * 1024 * 1024); // 100 MB;

        // let reg = registry_service_client::RegistryServiceClient::new(channel.clone());

        let user = user_service_client::UserServiceClient::new(channel);
        // let registry: registry_service_client::RegistryServiceClient<Channel> = reg.into();
        Ok(CliRegistryClient::new(reg, user))
    }
}

#[derive(Debug)]
pub struct CliRegistryClient {
    registry_client: registry_service_client::RegistryServiceClient<
        InterceptedService<Channel, AuthInterceptor>,
    >,
    users_client: user_service_client::UserServiceClient<Channel>,
}

impl CliRegistryClient {
    pub fn new(
        registry: registry_service_client::RegistryServiceClient<
            InterceptedService<Channel, AuthInterceptor>,
        >,
        users: user_service_client::UserServiceClient<Channel>,
    ) -> Self {
        Self {
            registry_client: registry,
            users_client: users,
        }
    }

    pub async fn publish(&mut self, publish_req: PublishRequest) -> anyhow::Result<()> {
        match self.registry_client.publish(publish_req.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => match e.code() {
                tonic::Code::AlreadyExists => {
                    let format = format!(
                        "{}:{}",
                        publish_req.clone().lib.unwrap().name,
                        publish_req.clone().lib.unwrap().version
                    );
                    Prompter::warning(&format!("{:?} -> {}", format, e.message()));
                    exit(1)
                }
                _ => Err(anyhow::anyhow!(e)),
            },
        }
    }

    // pub async fn download(
    //     &mut self,
    //     download_req: DownloadRequest,
    // ) -> anyhow::Result<plm_core::DownloadResponse> {
    //     let response = self
    //         .registry_client
    //         .download(download_req)
    //         .await.map_err(|e| anyhow::anyhow!(e))?;

    //     Ok(response.into_inner())
    // }

    pub async fn create_user(&mut self, login_req: LoginRequest) -> anyhow::Result<plm_core::User> {
        let create_user = plm_core::CreateUserRequest {
            password: login_req.token,
            username: login_req.username,
        };
        let response = self
            .users_client
            .create_user(create_user)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(response.into_inner())
    }

    pub async fn login(&mut self, login_req: LoginRequest) -> Result<LoginResponse, Status> {
        let response = self.users_client.login(login_req).await?;
        Ok(response.into_inner())
    }
}

#[async_trait]
impl plm_core::registry::Registry for CliRegistryClient {
    async fn download(
        &mut self,
        dep: plm_core::plm::library::v1::Dependency,
    ) -> anyhow::Result<plm_core::plm::library::v1::Library> {
        let response = self
            .registry_client
            .download(DownloadRequest {
                full_or_partial: Some(plm_core::FullOrPartial::Full(if dep.version.eq("") {
                    dep.library_id
                } else {
                    format!("{}:{}", dep.library_id, dep.version)
                })),
                ..Default::default()
            })
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        match response.into_inner().protobuf_or_gz {
            Some(res) => match res {
                ProtobufOrGz::Gz(_gz) => {
                    return Err(anyhow::anyhow!(format!(
                        "error while unpacking download response"
                    )))
                }
                ProtobufOrGz::Protobuf(pb) => Ok(pb),
            },
            None => {
                return Err(anyhow::anyhow!(format!(
                    "error while unpacking download response"
                )))
            }
        }
    }

    async fn publish(&self, _lib: plm_core::plm::library::v1::Library) -> anyhow::Result<()> {
        Ok(())
    }
}
