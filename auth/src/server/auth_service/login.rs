use sqlx::PgPool;
use thiserror::Error;

use crate::proto::auth::LoginRequest;

#[derive(Debug, Error)]
pub enum CheckUserExistsError {
    #[error("Provided credintels does not belong to any registered user")]
    NonExistingUser,
    #[error("Something went wrong in the DB: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub async fn check_user_exists(
    login_request: LoginRequest,
    db_pool: &PgPool,
) -> Result<(), CheckUserExistsError> {
    let query = sqlx::query!(
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

    match query {
        // the user was successfully added
        Some(_) => Ok(()),
        None => Err(CheckUserExistsError::NonExistingUser),
    }
}
