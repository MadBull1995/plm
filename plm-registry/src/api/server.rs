use std::{time::Duration, net::SocketAddr};

use tower::{ServiceBuilder, layer::{util::Stack, LayerFn}};
use tonic::{async_trait, transport::{Server as GrpcServer, Channel}};
use plm_core::RegistryServiceServer;

use crate::service::RegistryService;
#[derive(Debug, Clone, Copy)]
pub struct RegistryServerBuilder {
    addr: Option<SocketAddr>,
}

impl RegistryServerBuilder {
    pub fn new() -> Self {
        let addr = "127.0.0.1:7575".parse().unwrap();
        Self {
            addr: Some(addr)
        }
    }

    pub fn with_addr(&mut self, addr: String) -> &mut Self {
        self.addr = Some(addr.parse().unwrap());
        self
    }

    pub fn build(self) -> RegistryServer {

        RegistryServer { 
            addr: self.addr.unwrap(),
            service: RegistryService::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegistryServer {
    addr: SocketAddr,
    service: RegistryService,
}

impl RegistryServer {

    fn setup_layer(&self) -> Stack<tower::timeout::TimeoutLayer, Stack<tower::load_shed::LoadShedLayer, tower::layer::util::Identity>> {
        let layer = ServiceBuilder::new()
            .load_shed()
            .timeout(Duration::from_secs(30))
            .into_inner();
        layer
    }

    fn setup_service(&self) -> RegistryServiceServer<RegistryService> {
        RegistryServiceServer::new(self.service.clone())
    }

    pub async fn run(&self) {

        let layer = self.setup_layer();
        
        let registry_service = self.setup_service();
        let server = GrpcServer::builder()
            .layer(layer)
            .add_service(registry_service)
            .serve(self.addr).await;
        
        match server {
            Err(err) => panic!("registry server failed: {:?}", err),
            Ok(_) => println!("registry server exited")
        }
    }
}