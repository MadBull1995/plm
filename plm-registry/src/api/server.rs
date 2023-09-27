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

use std::{env, net::SocketAddr, sync::Arc};

// use tower::{ServiceBuilder, layer::{util::Stack, LayerFn}};
use plm_core::{registry_service_server, user_service_server};
use tonic::transport::Server as GrpcServer;
use tracing::{debug, warn};

use crate::{
    psql::QueryLayer,
    service::{RegistryService, UserService},
    RegistryStorage,
};

lazy_static::lazy_static! {
    pub static ref SECRET: String = env::var("PLM_SECRET").unwrap_or_else(|_| "default_secret".to_string());
}

#[derive(Clone)]
pub struct RegistryServerBuilder {
    addr: Option<SocketAddr>,
    storage: Arc<Box<dyn RegistryStorage + Send + Sync>>,
}

impl RegistryServerBuilder {
    pub fn new(storage: Box<dyn RegistryStorage + Send + Sync>) -> Self {
        let addr = "127.0.0.1:7575".parse().unwrap();
        Self {
            addr: Some(addr),
            storage: Arc::new(storage),
        }
    }

    pub fn with_addr(&mut self, addr: String) -> &mut Self {
        self.addr = Some(addr.parse().unwrap());
        self
    }

    pub fn build(self) -> RegistryServer {
        let query_layer = QueryLayer::new();
        let user = UserService {
            data: query_layer.clone(),
        };
        let registry = RegistryService {
            data: query_layer.clone(),
            storage: self.storage,
        };
        RegistryServer {
            addr: self.addr.unwrap(),
            registry,
            user,
        }
    }
}

#[derive(Clone)]
pub struct RegistryServer {
    addr: SocketAddr,
    registry: RegistryService,
    user: UserService,
}

impl RegistryServer {
    // fn setup_layer(&self) -> Stack<tower::timeout::TimeoutLayer, Stack<tower::load_shed::LoadShedLayer, tower::layer::util::Identity>> {
    //     let layer = ServiceBuilder::new()
    //         .load_shed()
    //         .timeout(Duration::from_secs(30))
    //         .into_inner();
    //     layer
    // }

    async fn setup_and_run(&self, mut server_builder: GrpcServer) {
        debug!("setting up services");
        let server = server_builder
            .add_service(
                registry_service_server::RegistryServiceServer::with_interceptor(
                    self.registry.clone(),
                    auth_guard,
                ),
            )
            .add_service(user_service_server::UserServiceServer::new(
                self.user.clone(),
            ))
            .serve(self.addr)
            .await;

        match server {
            Err(err) => panic!("registry server failed: {:?}", err),
            Ok(_) => println!("registry server exited"),
        }
    }

    pub async fn run(&self) {
        debug!("running gRPC server -> {}", self.addr);
        // let layer = self.setup_layer();
        let server_builder = GrpcServer::builder();

        self.setup_and_run(server_builder).await;
    }
}
use crate::utils::auth;

/// This function will get called on each inbound request, if a `Status`
/// is returned, it will cancel the request and return that status to the
/// client.
fn auth_guard(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    warn!("Intercepting request: {:?}", req);

    match req.metadata().get("authorization") {
        Some(t) => match extract_bearer_token(t.to_str().unwrap()) {
            None => Err(tonic::Status::unauthenticated(
                "Invalid token format should be: Bearer <token>".to_string(),
            )),
            Some(t) => {
                if auth::validate_jwt_token(t, SECRET.as_bytes()).is_ok() {
                    Ok(req)
                } else {
                    Err(tonic::Status::unauthenticated("No valid auth token"))
                }
            }
        },
        _ => Err(tonic::Status::unauthenticated(
            "Missing authorization metadata",
        )),
    }
}

fn extract_bearer_token(s: &str) -> Option<&str> {
    if s.starts_with("Bearer ") && s.len() > "Bearer ".len() {
        Some(&s["Bearer ".len()..])
    } else {
        None
    }
}
