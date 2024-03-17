use super::{compute_password_hash, RegisterData};
use anyhow::Context;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use secrecy::ExposeSecret;
use sqlx::{Executor, Postgres, Row, Transaction};
use tokio::task::spawn_blocking;

type UserId = i32;

// #[derive(thiserror::Error)]
// enum RegisterError {
//     Validation,
// }

// still temporary fake token
pub fn generate_auth_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

#[tracing::instrument(
    name = "Saving new user details into the database",
    skip(transaction, register_request)
)]
pub async fn register_user_into_db(
    transaction: &mut Transaction<'_, Postgres>,
    register_request: RegisterData,
) -> Result<UserId, anyhow::Error> {
    let password_hash =
        spawn_blocking(move || compute_password_hash(register_request.password.as_ref()))
            .await?
            .context("failed to hash password")?;

    let query = sqlx::query!(
        r#"INSERT INTO account
        (firstname, lastname, email, username, password_hash)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING user_id"#,
        register_request.firstname.as_ref(),
        register_request.lastname.as_ref(),
        register_request.email.as_ref(),
        register_request.username.as_ref(),
        password_hash.expose_secret(),
    );

    let row = transaction.fetch_one(query).await.map_err(|e| {
        tracing::error!("Failed to exectute query: {:?}", e);
        e
    })?;

    let user_id = row.get::<i32, _>("user_id");

    Ok(user_id)
}

#[tracing::instrument(
    name = "Store auth_token into DB"
    skip(transaction, auth_token)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    auth_token: &str,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO auth_tokens (auth_token, user_id)
        VALUES ($1, $2)
        "#,
        auth_token,
        user_id
    );

    transaction.execute(query).await.map_err(|e| {
        tracing::error!("Failed to exectute query: {:?}", e);
        e
    })?;

    Ok(())
}
