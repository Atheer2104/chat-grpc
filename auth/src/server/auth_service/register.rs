use super::{compute_password_hash, RegisterData};
use anyhow::Context;

use secrecy::ExposeSecret;
use sqlx::{Executor, Postgres, Row, Transaction};
use std::sync::Arc;
use tokio::task::spawn_blocking;

// #[derive(thiserror::Error)]
// enum RegisterError {
//     Validation,
// }

#[tracing::instrument(
    name = "Saving new user details into the database",
    skip(transaction, register_request_arc)
)]
pub async fn register_user_into_db(
    transaction: &mut Transaction<'_, Postgres>,
    register_request_arc: Arc<RegisterData>,
) -> Result<i32, anyhow::Error> {
    let register_request = register_request_arc.clone();

    let password_hash = spawn_blocking(move || {
        compute_password_hash(register_request_arc.clone().password.as_ref())
    })
    .await?
    .context("failed to hash password")?;

    // let password_hash =
    //     compute_password_hash(password_arc.clone().as_ref()).context("failed to hash password")?;

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
