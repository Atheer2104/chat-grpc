use secrecy::Secret;
use sqlx::PgPool;
use thiserror::Error;
use tokio::task::spawn_blocking;

use crate::proto::auth::LoginRequest;

use super::verify_password_hash;

#[derive(Debug, Error)]
pub enum CheckUserExistsError {
    #[error("Provided credintels does not belong to any registered user")]
    NonExistingUser,
    #[error("Provided password is wrong")]
    WrongPassword,
    #[error("Something went wrong in the DB: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    UnexpectedError(#[from] tokio::task::JoinError),
}

#[tracing::instrument(name = "Get stored password hash", skip(username, db_pool))]
pub async fn get_stored_password_hash(
    username: &str,
    db_pool: &PgPool,
) -> Result<Option<(i32, Secret<String>)>, sqlx::Error> {
    let query = sqlx::query!(
        r#"SELECT user_id, password_hash from account WHERE username = $1"#,
        username
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to retrieve password hash: {:?}", e);
        e
    })?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));

    // tracing::info!("getting stored passwordh hash query: {:?}", query);

    Ok(query)
}

#[tracing::instrument(name = "checking if user exists", skip(db_pool, login_request))]
pub async fn check_user_exists(
    login_request: LoginRequest,
    db_pool: &PgPool,
) -> Result<i32, CheckUserExistsError> {
    let mut user_id = None;
    // some dummy value
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
            gZiV/M1gPc22ElAH/Jh1Hw$\
            CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_password_hash(login_request.username.as_ref(), db_pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    if user_id.is_none() {
        return Err(CheckUserExistsError::NonExistingUser);
    }

    let result_verifying_password = spawn_blocking(move || {
        verify_password_hash(expected_password_hash, login_request.password)
    })
    .await
    .map_err(CheckUserExistsError::UnexpectedError)?;

    match result_verifying_password {
        Ok(_) => {}
        Err(_) => return Err(CheckUserExistsError::WrongPassword),
    }

    Ok(user_id.unwrap())
}
