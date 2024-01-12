use sqlx::PgPool;

use crate::proto::auth::LoginRequest;

pub async fn check_user_exists(
    login_request: LoginRequest,
    db_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"SELECT username, password FROM account
       WHERE username = $1 AND password = $2"#,
        login_request.username,
        login_request.password
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to exectute query {:?}", e);
        e
    })?;

    // this is wrong because we have to check that the result value ie is valid i there exist a row currently we do not

    Ok(())
}
