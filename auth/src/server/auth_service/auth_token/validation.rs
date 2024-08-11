use std::collections::BTreeMap;

use anyhow::Error;
use chrono::Local;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use secrecy::{ExposeSecret, Secret};
use sha2::Sha512;

pub fn auth_token_expired(secret_key: Secret<String>, auth_token: &str) -> Result<bool, Error> {
    let key: Hmac<Sha512> = Hmac::new_from_slice(secret_key.expose_secret().as_bytes())?;

    let claims: BTreeMap<String, String> = auth_token.verify_with_key(&key).unwrap();

    let now_timestamp = Local::now().timestamp().to_string();
    if now_timestamp > claims["exp"] {
        return Ok(true);
    }

    Ok(false)
}
