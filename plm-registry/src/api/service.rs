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

use std::sync::Arc;

use diesel::result::{DatabaseErrorKind, Error};
use plm_core::{
    registry_service_server, user_service_server,
    utils::auth,
    CreateUserRequest, DownloadRequest, DownloadResponse, FullOrPartial, Library, LoginRequest,
    LoginResponse, PublishRequest, User,
};
use tonic::{async_trait, Request, Response, Status};
use tracing::{debug, error, info, warn};

use crate::{
    api::server::SECRET, models::NewVersion, psql::QueryLayer, RegistryStorage,
};

#[derive(Clone)]
pub struct RegistryService {
    pub(crate) data: QueryLayer,
    pub(crate) storage: Arc<Box<dyn RegistryStorage + Sync + Send>>,
}

#[async_trait]
impl registry_service_server::RegistryService for RegistryService {
    async fn download(
        &self,
        request: Request<DownloadRequest>,
    ) -> Result<Response<DownloadResponse>, tonic::Status> {
        let lib_req = request.into_inner();
        info!("download lib: {:?}", lib_req.clone());

        match lib_req.full_or_partial {
            None => {
                return Err(tonic::Status::invalid_argument(
                    "must specify a download request full/partial".to_string()
                ));
            }
            Some(r) => {
                match r {
                    FullOrPartial::Full(full) => {
                        let latest =
                            self.data
                                .get_latest_version_for_lib(&full)
                                .await
                                .map_err(|e| {
                                    tonic::Status::not_found(format!(
                                        "failed to fetch library: {} - {:?}",
                                        full, e
                                    ))
                                })?;
                        match latest {
                            Some(latest) => {
                                let release = self
                                    .data
                                    .get_release(&full, Some(latest.max_version_id), None)
                                    .await
                                    .map_err(|e| {
                                        tonic::Status::internal(format!(
                                            "error on fetching library: {:?}",
                                            e
                                        ))
                                    })?;
                                // self.data.get_release(, lib_version, lib_scope)
                                let mut lib = Library::default();
                                match release {
                                    None => Err(tonic::Status::not_found(format!(
                                        "library release not found: {}",
                                        &full
                                    ))),
                                    Some(mut version) => {
                                        lib.name = version.0.name;
                                        lib.version =
                                            version.1.pop().unwrap().version_number.clone();
                                        let mut lib_full_path = String::new();
                                        lib_full_path
                                            .push_str(&format!("{}/{}", lib.name, lib.version));
                                        println!("{:?}", lib_full_path);
                                        // TODO: Handle library file parsing
                                        let files =
                                            self.storage.load(&lib_full_path).map_err(|e| {
                                                tonic::Status::internal(format!(
                                                    "failed to fetch proto files: {}",
                                                    e
                                                ))
                                            })?;
                                        let mut downloaded_lib = DownloadResponse::default();
                                        lib.packages.push(plm_core::Package {
                                            files,
                                            ..Default::default()
                                        });
                                        downloaded_lib.protobuf_or_gz =
                                            Some(plm_core::ProtobufOrGz::Protobuf(lib));
                                        Ok(Response::new(downloaded_lib))
                                    }
                                }
                            }
                            None => Err(tonic::Status::unimplemented(format!(
                                "not implemented yet to install a pinned version: {}",
                                &full
                            ))),
                        }
                    }
                    FullOrPartial::Partial(_partial) => {
                        return Err(tonic::Status::unimplemented("not implemented yet".to_string()));
                    }
                }
            }
        }
    }

    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let pub_req = request.into_inner().lib.unwrap();
        info!("publish lib: {:?} : {}", pub_req.name, pub_req.version);

        let release = self
            .data
            .get_release(&pub_req.name, None, None)
            .await
            .map_err(|e| tonic::Status::internal(format!("error on fetching library: {:?}", e)))?;
        info!("{:?}", release);
        let mut conn = self.data.conn.lock().await;
        let transaction = conn.build_transaction().run(|c| {
            #[allow(unused_assignments)]
            let mut library = None;
            match release {
                Some(r) => {
                    library = Some(r.0);
                }
                None => {
                    let release = self.data.create_release(&pub_req, c)?;
                    library = Some(release);
                }
            }

            self.storage.save(pub_req.clone()).map_err(|e| {
                error!("{:?}", e);
                diesel::result::Error::RollbackTransaction
            })?;
            let new_version = NewVersion {
                library_id: library.as_ref().unwrap().lib_id,
                version_number: &pub_req.version,
            };
            let version = self.data.create_version(&new_version, c).map_err(|e| {
                error!("{:?}", e);
                diesel::result::Error::RollbackTransaction
            })?;

            println!("{:?}", version);

            // TODO: Add deps

            Ok(library)
        });

        match transaction {
            Err(Error::DatabaseError(kind, info)) => match kind {
                DatabaseErrorKind::UniqueViolation => {
                    warn!("{:?}", info);
                    Err(Status::already_exists(
                        "release is already exists, try to publish with a differnt version.".to_string()
                    ))
                }
                _ => Err(Status::internal(format!(
                    "some error occured during db session: {:?}",
                    kind
                ))),
            },
            Err(e) => Err(Status::internal(format!(
                "some error occured during db session: {:?}",
                e
            ))),
            Ok(r) => {
                println!("{:?}", r);

                Ok(Response::new(()))
            }
        }
    }
}

#[derive(Clone)]
pub struct UserService {
    pub(crate) data: QueryLayer,
}

#[async_trait]
impl user_service_server::UserService for UserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<User>, Status> {
        let mut u = request.into_inner();
        info!("create user: {}", u.username.clone());
        u.password = auth::Argon2Helper::hash_password(&u.password)
            .map_err(|err| Status::internal(format!("failed to hash user password: {}", err)))?;

        let new_db_user = self
            .data
            .create_user(&u)
            .await
            .map_err(|err| Status::internal(format!("failed to create new user: {:?}", err)))?;

        let response = plm_core::User {
            username: u.username,
            ..Default::default()
        };

        // response.created_at = Some(timestamp);
        debug!("{:?}: {}", new_db_user.user_id, new_db_user.username);
        Ok(Response::new(response))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<plm_core::LoginResponse>, Status> {
        let login_req = request.into_inner();
        info!("login for user: {}", login_req.username.clone());

        let user = self.data.get_user(&login_req.username).await.map_err(|e| {
            Status::internal(format!(
                "failed to get user: {} - {:?}",
                login_req.username, e
            ))
        })?;
        match user {
            Some(u) => {
                info!("fetched password: {}", u.password_hash);
                let verify = auth::Argon2Helper::verify_password(login_req.token, u.password_hash)
                    .map_err(|e| {
                        Status::internal(format!(
                            "failed to verify user {} password - {:?}",
                            login_req.username, e
                        ))
                    })?;
                if verify {
                    match crate::auth::create_jwt_token(SECRET.as_bytes(), &u.user_id) {
                        Err(e) => Err(Status::internal(format!("failed to generate token: {}", e))),
                        Ok(jwt) => {
                            let res = LoginResponse {
                                token: jwt
                            };
                            Ok(Response::new(res))
                        }
                    }
                } else {
                    Err(Status::unauthenticated("invalid password"))
                }
            }
            None => Err(Status::not_found("username not exists")),
        }
    }
    // async fn login(&self,request:Request<LoginRequest>) -> Result<Response<()> ,Status> {
    //     Ok(Response::new(());
    // }
}
