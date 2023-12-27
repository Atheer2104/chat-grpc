use sqlx::postgres::PgPool;
use tonic::{Request, Response, Status};

// bring in our messages
use crate::proto::auth::auth_server::Auth;
use crate::proto::auth::{LoginRequest, RegisterRequest, Token};

#[derive(Debug)]
pub struct AuthenticationService {
    pub pool: PgPool,
}

#[tonic::async_trait]
impl Auth for AuthenticationService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let login_request = request.into_inner();

        sqlx::query!(
            r#"SELECT username, password FROM account
           WHERE username = $1 AND password = $2"#,
            login_request.username,
            login_request.password
        )
        .fetch_one(&self.pool)
        .await
        .expect("something went wrong");

        let token = Token {
            access_token: "654321".into(),
        };

        Ok(Response::new(token))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<Token>, Status> {
        let register_request = request.into_inner();

        // TODO have custom error messages
        sqlx::query!(
            r#"INSERT INTO account
            (firstname, lastname, email, username, password)
            VALUES ($1, $2, $3, $4, $5)"#,
            register_request.firstname,
            register_request.lastname,
            register_request.email,
            register_request.username,
            register_request.password,
        )
        .execute(&self.pool)
        .await
        .expect("something went wrong");

        let token = Token {
            access_token: "123456".into(),
        };

        Ok(Response::new(token))
    }
}
