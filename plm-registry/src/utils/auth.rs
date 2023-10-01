// Copyright 2023 PLM Authors
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

use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt_token(
    secret: &[u8],
    user_id: &i32,
) -> Result<String, jsonwebtoken::errors::Error> {
    // Get current time in seconds since the Unix epoch
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Set the token to expire in 24 hour (3600 seconds * 24)
    let expiration: usize = (since_the_epoch.as_secs() + (3600 * 24))
        .try_into()
        .unwrap();
    let claims = Claims {
        sub: format!("{}", user_id.to_owned()),
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn validate_jwt_token(token: &str, secret: &[u8]) -> Result<(), jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(())
}
