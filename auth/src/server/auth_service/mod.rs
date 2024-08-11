mod auth_token;
mod check_existing_user;
mod password;
mod register;

pub use auth_token::*;
pub use check_existing_user::*;
pub use password::*;
pub use register::*;

use redis::aio::MultiplexedConnection;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;
use tonic::{Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

use crate::proto::auth::auth_server::Auth;
// bring in our messages
use crate::proto::auth::{LoginRequest, RegisterRequest, Token};
use crate::secrets::Secrets;

pub use super::RegisterData;

pub type RedisCon = Arc<Mutex<MultiplexedConnection>>;

pub struct AuthenticationService {
    pub db_pool: PgPool,
    pub redis_con: RedisCon,
    pub secrets: Secrets,
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

        if error_details.has_bad_request_violations() {
            let status = Status::with_error_details(
                Code::InvalidArgument,
                "Request has invalid argumetns",
                error_details,
            );

            return Err(status);
        }

        let user_id = match check_user_exists(login_request, &self.db_pool).await {
            Ok(e) => e,
            Err(e) => match e {
                CheckUserExistsError::NonExistingUser => {
                    return Err(Status::unauthenticated(e.to_string()))
                }
                _ => return Err(Status::internal(e.to_string())),
            },
        };

        let mut auth_token = match get_token_redis(self.redis_con.clone(), &user_id).await {
            Ok(e) => e,
            Err(_) => match get_token_db(&self.db_pool, &user_id).await {
                Ok(e) => match e {
                    Some(token) => token,
                    None => return Err(Status::internal("Couldn't get auth token from DB")),
                },
                Err(_) => return Err(Status::internal("Couldn't get auth token")),
            },
        };

        let secret_key = self.secrets.jwt_secret.clone();
        auth_token = match auth_token_expired(secret_key.clone(), auth_token.as_str()) {
            Ok(token_expired) => {
                if token_expired {
                    tracing::info!("auth token has expired");
                    let new_auth_token = match spawn_blocking(move || {
                        generate_auth_token(secret_key, user_id.to_string().as_str())
                    })
                    .await
                    {
                        Ok(res) => match res {
                            Ok(e) => e,
                            Err(_) => {
                                return Err(Status::internal("Failed to generate auth token"))
                            }
                        },
                        Err(_) => return Err(Status::internal("Could not create a auth token")),
                    };

                    if update_token_db(&self.db_pool, &user_id, &new_auth_token)
                        .await
                        .is_err()
                    {
                        return Err(Status::internal(
                            "auth token was invalid and could not update auth token in DB",
                        ));
                    }

                    if store_token_redis(self.redis_con.clone(), &user_id, &new_auth_token)
                        .await
                        .is_err()
                    {
                        return Err(Status::internal("Could not store auth token into redis"));
                    }

                    new_auth_token
                } else {
                    // returning original auth_token
                    auth_token
                }
            }
            Err(_) => return Err(Status::internal("Couldn't check expire time of auth token")),
        };

        let token = Token {
            access_token: auth_token,
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
        // let register_request_arc = Arc::new(reqister_request);

        let mut transaction = match self.db_pool.begin().await {
            Ok(transaction) => transaction,
            Err(_) => {
                return Err(Status::internal(
                    "Could not create a transaction for postgresql",
                ))
            }
        };

        let user_id = match register_user_into_db(&mut transaction, reqister_request).await {
            Err(_) => return Err(Status::internal("Could not retrieve user_id")),
            Ok(user_id) => user_id,
        };

        let secret_key = self.secrets.jwt_secret.clone();
        let auth_token = match spawn_blocking(move || {
            generate_auth_token(secret_key, user_id.to_string().as_str())
        })
        .await
        {
            Ok(res) => match res {
                Ok(e) => e,
                Err(_) => return Err(Status::internal("Failed to generate auth token")),
            },
            Err(_) => return Err(Status::internal("Could not create a auth token")),
        };

        if store_token_db(&mut transaction, &user_id, &auth_token)
            .await
            .is_err()
        {
            return Err(Status::internal("Could not store auth token into DB"));
        }

        match transaction.commit().await {
            Ok(_) => match store_token_redis(self.redis_con.clone(), &user_id, &auth_token).await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to save auth token to redis {:?}", e);
                    return Err(Status::internal("Could not store auth token into redis"));
                }
            },
            Err(_) => {
                return Err(Status::internal(
                    "Could not commit user register transaction",
                ));
            }
        };

        let token = Token {
            access_token: auth_token,
        };

        Ok(Response::new(token))
    }
}
