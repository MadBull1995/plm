use plm_core::{DownloadRequest, RegistryServiceClient};
use tonic::{async_trait, transport::Channel};

use crate::utils::errors::{PlmResult, PlmError};

#[derive(Debug)]
pub struct CliRegistryClientBuilder {
    addr: Option<String>,
}

impl CliRegistryClientBuilder {
    pub fn new() -> Self {
        Self {
            addr: Some("[::1]:7575".to_string()),
        }
    }

    pub fn with_addr(&mut self, addr: &String) -> &mut Self {
        let socket_addr = addr.parse().unwrap();
        self.addr = Some(socket_addr);

        self
    }

    pub async fn build(self) -> PlmResult<CliRegistryClient> {
        let addr = self.addr.unwrap().clone();
        let channel = RegistryServiceClient::connect(addr).await.map_err(|err| {
            PlmError::InternalError("Failed to communicate with registry server".to_string())
        })?;
        Ok(CliRegistryClient::new(channel))
    }
}

#[derive(Debug)]
pub struct CliRegistryClient {
    client: RegistryServiceClient<Channel>,
}

impl CliRegistryClient {
    pub fn new(channel: RegistryServiceClient<Channel>) -> Self {
        Self { client: channel }
    }

    pub async fn download(
        &mut self,
        download_req: DownloadRequest,
    ) -> PlmResult<plm_core::DownloadResponse> {
        let response = self.client.download(download_req).await.map_err(|err| {
            PlmError::InternalError(format!("Failed to download library: {:?}", err))
        })?;

        Ok(response.into_inner())
    }
}
