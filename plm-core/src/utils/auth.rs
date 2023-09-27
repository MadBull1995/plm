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

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, ParamsBuilder,
};

pub struct Argon2Helper;

impl Argon2Helper {
    pub fn hash_password(password: &str) -> anyhow::Result<String> {
        // Generate a random salt
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argo2id();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| anyhow::anyhow!(err.to_string()))?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(password: String, hash: String) -> anyhow::Result<bool> {
        let argon2 = argo2id();
        let parsed_hash =
            PasswordHash::new(&hash).map_err(|err| anyhow::anyhow!(err.to_string()))?;
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

fn argo2id() -> Argon2<'static> {
    let params = ParamsBuilder::default().build().unwrap();
    // let p = params
    //     .t_cost(3)
    //     .m_cost(65536)
    //     .p_cost(4)
    //     .output_len(32)
    //     .build().unwrap();

    Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x10, params)
}
