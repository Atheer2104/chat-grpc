mod login;
mod register;

pub use login::*;
pub use register::*;

use sqlx::postgres::PgPool;
use tonic::{Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

use crate::proto::auth::auth_server::Auth;
// bring in our messages
use crate::proto::auth::{LoginRequest, RegisterRequest, Token};

pub use super::RegisterData;

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

        let mut error_details = ErrorDetails::new();

        if login_request.username.is_empty() {
            error_details.add_bad_request_violation("username", "username field is empty");
        }

        if login_request.password.is_empty() {
            error_details.add_bad_request_violation("password", "password field is empty");
        }

        if check_user_exists(login_request, &self.db_pool)
            .await
            .is_err()
        {
            error_details.set_localized_message("en-US", "The login information you provided is not valid. Please check your username and password and try again.");
        }

        if error_details.has_bad_request_violations() {
            let status = Status::with_error_details(
                Code::InvalidArgument,
                "Request has invalid argumetns",
                error_details,
            );

            return Err(status);
        }

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
        // this is needed for type annotation
        let request_result: Result<RegisterData, _> = request.into_inner().try_into();

        let reqister_request = match request_result {
            Err(e) => {
                let error_details =
                    ErrorDetails::with_bad_request_violation(e.field, e.message.to_string());

                let status = Status::with_error_details(
                    Code::InvalidArgument,
                    "bad request, Invalid arguments",
                    error_details,
                );

                return Err(status);
            }
            Ok(s) => s,
        };

        let mut transaction = match self.db_pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return Err(Status::internal(
                    "Could not create a transaction for postgresql",
                ))
            }
        };

        let user_id = match register_user_into_db(&mut transaction, &reqister_request).await {
            Err(_) => return Err(Status::internal("Could not retrieve user_id")),
            Ok(user_id) => user_id,
        };

        let auth_token = generate_auth_token();
        if store_token(&mut transaction, user_id, &auth_token)
            .await
            .is_err()
        {
            return Err(Status::internal("Could not store auth token into DB"));
        }

        if transaction.commit().await.is_err() {
            return Err(Status::internal(
                "Could not commit user register transaction",
            ));
        }

        let token = Token {
            access_token: auth_token,
        };

        Ok(Response::new(token))
    }
}
