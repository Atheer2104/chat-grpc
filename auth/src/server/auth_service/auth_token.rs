use chrono::{Duration, Local};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use secrecy::{ExposeSecret, Secret};
use sha2::Sha512;
use std::collections::BTreeMap;
use std::sync::Arc;

use super::RegisterData;

// still temporary fake token
pub fn generate_auth_token(
    secret_key: Secret<String>,
    user_id: &str,
    register_request_arc: Arc<RegisterData>,
) -> Result<String, anyhow::Error> {
    let register_request = register_request_arc.clone();

    let key: Hmac<Sha512> = Hmac::new_from_slice(secret_key.expose_secret().as_bytes())?;
    let mut claims = BTreeMap::new();

    // generic claims
    claims.insert("iss", "Chat-gRPC");
    claims.insert("sub", "auth token");

    let now_timestamp = Local::now().timestamp().to_string();
    // issued at
    claims.insert("iat", now_timestamp.as_str());

    let one_week_from_now_timestamp = (Local::now() + Duration::weeks(1)).timestamp().to_string();
    // expiration time
    claims.insert("exp", one_week_from_now_timestamp.as_str());

    // application claims
    claims.insert("user_id", user_id);
    claims.insert("username", register_request.username.as_ref());
    claims.insert("email", register_request.email.as_ref());
    let token = claims.sign_with_key(&key)?;

    Ok(token)
}
