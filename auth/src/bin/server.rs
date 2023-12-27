use auth::configuration::get_configuration;
use sqlx::postgres::PgPool;
use std::error::Error;
use tonic::transport::Server;

use auth::proto::auth::auth_server::AuthServer;
use auth::proto::auth::FILE_DESCRIPTOR_SET;
use auth::server::AuthenticationService;

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
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(AuthServer::new(auth))
        .add_service(reflection_service)
        .serve(address)
        .await?;

    Ok(())
}
