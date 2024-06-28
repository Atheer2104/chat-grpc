use std::collections::BTreeMap;

use crate::secret::get_secrets;
use chrono::Local;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use secrecy::ExposeSecret;
use sha2::Sha512;
use tonic::{Request, Status};

macro_rules! check_claim_key {
    ($claims:expr, $claim_key:expr, $true_claim:expr, $error_msg:expr) => {
        if !$claims.contains_key($claim_key) {
            return Err(Status::invalid_argument(format!(
                "JWT auth token does not contain {} key",
                $claim_key
            )));
        }

        if $claims[$claim_key] != $true_claim {
            return Err(Status::unauthenticated($error_msg));
        }
    };
}

macro_rules! check_claim_expired {
    ($claim:expr, $error_msg:expr) => {
        let now_timestamp = Local::now().timestamp().to_string();
        if now_timestamp > $claim {
            return Err(Status::unauthenticated($error_msg));
        }
    };
}

pub fn auth_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    println!("auth interceptor executed");

    match req.metadata().get("authorization") {
        Some(t) => {
            println!("received value: {:?}", t);

            tracing::info!("Reading secrets");

            let secrets = match get_secrets() {
                Ok(s) => s,
                Err(_) => return Err(Status::internal("couldn't read auth secret key")),
            };

            let key: Hmac<Sha512> =
                Hmac::new_from_slice(secrets.jwt_secret.expose_secret().as_bytes()).unwrap();

            let bearer_token = match t.to_str() {
                Ok(t) => t,
                Err(_) => {
                    return Err(Status::invalid_argument(
                        "the provided jwt token does not contain valid ASCII chars",
                    ))
                }
            };

            let mut token_split_stream = bearer_token.split_whitespace();
            token_split_stream.next();
            let token_str = match token_split_stream.next() {
                Some(t) => t,
                None => return Err(Status::internal("Couldn't destructure the auth token")),
            };

            println!("token str: {}", token_str);

            let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();
            println!("{:?}", claims);

            check_claim_key!(&claims, "iss", "Chat-gRPC", "JWT iss does not match");
            check_claim_key!(&claims, "sub", "auth token", "JWT sub doese not match");
            check_claim_expired!(claims["exp"], "Auth token has expired");

            Ok(req)
        }
        None => Err(Status::unauthenticated("no valid auth token")),
    }
}
