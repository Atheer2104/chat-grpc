use sqlx::postgres::PgPool;
use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;
use tonic_health::server::HealthReporter;

use crate::proto::auth::auth_server::AuthServer;
use crate::proto::auth::FILE_DESCRIPTOR_SET;
use crate::server::AuthenticationService;

pub async fn run_server(
    connection_pool: PgPool,
    address: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let auth = AuthenticationService {
        db_pool: connection_pool,
    };

    // ! for some reason the health service is not working look into it later
    //
    // setting up health service
    // this will create a HealthReporter and HealthServer pair
    // let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    // health_reporter
    //     .set_serving::<AuthServer<AuthenticationService>>()
    //     .await;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        // .add_service(health_service)
        .add_service(AuthServer::new(auth))
        .add_service(reflection_service)
        .serve(address)
        .await?;

    Ok(())
}
