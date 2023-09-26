use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt_token(secret: &[u8], user_id: &i32) -> Result<String, jsonwebtoken::errors::Error> {
    // Get current time in seconds since the Unix epoch
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Set the token to expire in 1 hour (3600 seconds)
    let expiration: usize = (since_the_epoch.as_secs() + 3600).try_into().unwrap();
    let claims = Claims {
        sub: format!("{}", user_id.to_owned()),
        exp: expiration,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}


pub fn validate_jwt_token(token: &str, secret: &[u8]) -> Result<(), jsonwebtoken::errors::Error> {
    decode::<Claims>(&token, &DecodingKey::from_secret(secret), &Validation::default())?;
    Ok(())
}
