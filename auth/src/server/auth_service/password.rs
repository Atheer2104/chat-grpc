use crate::configuration::get_configuration;
use anyhow::{anyhow, Context};
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[tracing::instrument(
    name = "Validating password hash",
    skip(expected_password_hash, password_candidate)
)]
pub fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: String,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse password hash in PHC format")?;

    // here the params in the phc format will be used instead
    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

pub fn compute_password_hash(password: &str) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let argon_configuration = get_configuration()
        .expect("Failed to get configuration")
        .argon;

    let algorithm = match argon_configuration.variant.as_str() {
        "Argon2id" => Algorithm::Argon2id,
        "Argon2i" => Algorithm::Argon2i,
        "Argon2d" => Algorithm::Argon2d,
        _ => return Err(anyhow!("wrong Argon2 variant")),
    };

    let password_hash = Argon2::new(
        algorithm,
        Version::V0x13,
        Params::new(
            argon_configuration.memory,
            argon_configuration.iterations,
            argon_configuration.parallelism,
            None,
        )
        .unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

    Ok(Secret::new(password_hash))
}
