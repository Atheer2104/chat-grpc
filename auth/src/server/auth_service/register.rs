use super::{compute_password_hash, RedisCon, RegisterData};
use anyhow::{anyhow, Context};
use redis::{AsyncCommands, RedisResult};
use secrecy::ExposeSecret;
use sqlx::{Executor, PgPool, Postgres, Row, Transaction};
use std::sync::Arc;
use tokio::task::spawn_blocking;

type UserId = i32;

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
) -> Result<UserId, anyhow::Error> {
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

#[tracing::instrument(
    name = "get auth_token from db"
    skip(db_pool, )
)]
pub async fn get_token(db_pool: &PgPool, user_id: &UserId) -> Result<Option<String>, sqlx::Error> {
    let auth_token = sqlx::query!(
        r#"SELECT auth_token from auth_tokens WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to retrieve auth token: {:?}", e);
        e
    })?
    .map(|row| row.auth_token);

    Ok(auth_token)
}

#[tracing::instrument(
    name = "Store auth_token into redis"
    skip(redis_connection, auth_token)
)]
pub async fn store_token_redis(
    redis_connection: RedisCon,
    user_id: &UserId,
    auth_token: &str,
) -> Result<(), anyhow::Error> {
    let mut redis_con = redis_connection.lock().await;

    let res: RedisResult<()> = redis_con.set(user_id, auth_token).await;

    if res.is_err() {
        return Err(anyhow!("couldn't save auth token in redis"));
    }

    Ok(())
}

#[tracing::instrument(
    name = "get auth_token into redis"
    skip(redis_connection, user_id)
)]
pub async fn get_token_redis(
    redis_connection: RedisCon,
    user_id: &UserId,
) -> Result<String, anyhow::Error> {
    let mut redis_con = redis_connection.lock().await;

    let token = match redis_con.get(user_id).await {
        Ok(token) => token,
        Err(_) => return Err(anyhow!("couldn't get auth token in redis")),
    };

    Ok(token)
}
