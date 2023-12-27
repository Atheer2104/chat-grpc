use auth::configuration::get_configuration;
use sqlx::postgres::PgPool;
use std::error::Error;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod auth_proto {
    include!("authentication.rs");

    // used for server reflection
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}

// bring in our messages
use auth_proto::auth_server::{Auth, AuthServer};
use auth_proto::{LoginRequest, RegisterRequest, Token};

#[derive(Debug)]
struct AuthenticationService {
    pool: PgPool,
}

#[tonic::async_trait]
impl Auth for AuthenticationService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<Token>, Status> {
        let login_request = request.into_inner();

        // sqlx::query!(r#"SELECT ")

        unimplemented!()
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuartion = get_configuration().expect("Failed to read config file");
    let connection_pool = PgPool::connect(&configuartion.database.connection_string())
        .await
        .expect("failed to connect to postgres");

    let address = format!("[::1]:{}", configuartion.application_port).parse()?;
    let auth = AuthenticationService {
        pool: connection_pool,
    };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(auth_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(AuthServer::new(auth))
        .add_service(reflection_service)
        .serve(address)
        .await?;

    Ok(())
}
