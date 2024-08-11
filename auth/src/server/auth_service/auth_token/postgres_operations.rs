use sqlx::{Executor, PgPool, Postgres, Transaction};

#[tracing::instrument(
    name = "Store auth_token into DB"
    skip(transaction, auth_token)
)]
pub async fn store_token_db(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &i32,
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
    name = "update auth_token in DB"
    skip(pg_pool, auth_token)
)]
pub async fn update_token_db(
    pg_pool: &PgPool,
    user_id: &i32,
    auth_token: &str,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
        UPDATE auth_tokens SET auth_token = $1 WHERE user_id = $2
        "#,
        auth_token,
        user_id
    );

    pg_pool.execute(query).await.map_err(|e| {
        tracing::error!("Failed to exectute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(
    name = "get auth_token from db"
    skip(db_pool, )
)]
pub async fn get_token_db(db_pool: &PgPool, user_id: &i32) -> Result<Option<String>, sqlx::Error> {
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
