use sqlx::postgres::PgPool;
use tonic::{Request, Response, Status};

use crate::proto::auth::auth_server::Auth;
// bring in our messages
use crate::proto::auth::{LoginRequest, RegisterRequest, Token};

use super::UserRegisterSignupData;

#[derive(Debug)]
pub struct AuthenticationService {
    pub db_pool: PgPool,
}

#[tonic::async_trait]
impl Auth for AuthenticationService {
    // the % means that fmt::Display will be used as the format
    #[tracing::instrument(
        name = "User Login"
        skip(self, request)
        fields(
            username = %request.get_ref().username
        )
    )]
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let login_request = request.into_inner();

        if login_request.username.is_empty() {
            return Err(Status::invalid_argument("missing username field"));
        }

        if login_request.password.is_empty() {
            return Err(Status::invalid_argument("missing password field"));
        }

        sqlx::query!(
            r#"SELECT username, password FROM account
           WHERE username = $1 AND password = $2"#,
            login_request.username,
            login_request.password
        )
        // this should be changed to fetch_optional this can be done latter for propper error handeling
        .fetch_one(&self.db_pool)
        .await
        .expect("something went wrong");

        let token = Token {
            access_token: "654321".into(),
        };

        Ok(Response::new(token))
    }

    #[tracing::instrument(
        name = "Registering a new user"
        skip(self, request)
        fields(
            username = %request.get_ref().username,
            email = %request.get_ref().email
        )
    )]
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<Token>, Status> {
        let reqister_request: UserRegisterSignupData = match request.into_inner().try_into() {
            Err(_) => return Err(Status::invalid_argument("Invalid user details")),
            Ok(s) => s,
        };

        match register_user_into_db(&self.db_pool, &reqister_request).await {
            Err(_) => (),
            Ok(_) => (),
        };

        let token = Token {
            access_token: "123456".into(),
        };

        Ok(Response::new(token))
    }
}

#[tracing::instrument(
    name = "Saving new user details into the database",
    skip(db_pool, register_request)
)]
async fn register_user_into_db(
    db_pool: &PgPool,
    register_request: &UserRegisterSignupData,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO account
        (firstname, lastname, email, username, password)
        VALUES ($1, $2, $3, $4, $5)"#,
        register_request.firstname.as_ref(),
        register_request.lastname.as_ref(),
        register_request.email.as_ref(),
        register_request.username.as_ref(),
        register_request.password.as_ref(),
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to exectute query: {:?}", e);
        e
    })?;

    Ok(())
}
