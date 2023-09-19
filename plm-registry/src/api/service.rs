use plm_core::{RegistryService as RegistryServiceExt, DownloadRequest, DownloadResponse, Library};
use tonic::{async_trait, Request, Response, Status};

#[derive(Debug, Clone, Default)]
pub struct RegistryService {

}

#[async_trait]
impl RegistryServiceExt for RegistryService {
    async fn download(&self,request: Request<DownloadRequest>) -> Result<Response<DownloadResponse> ,tonic::Status, > {
        
        let mut lib = Library::default();
        

        let mut downloaded_lib = DownloadResponse::default();
        downloaded_lib.protobuf_or_gz = Some(
            plm_core::ProtobufOrGz::Protobuf(lib)
        );

        Ok(Response::new(downloaded_lib))
    }
}