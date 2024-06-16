#![allow(unused_imports)]

use std::sync::Arc;
use tokio::sync::Mutex;

use redis::aio::MultiplexedConnection;
use redis::Client;
use sqlx::postgres::PgPool;

use tonic::transport::{server::Router, Server};
use tonic_health::server::HealthReporter;

use crate::proto::auth::auth_server::AuthServer;
use crate::proto::auth::FILE_DESCRIPTOR_SET;
use crate::secrets::Secrets;
use crate::server::AuthenticationService;

pub fn build_server(
    connection_pool: PgPool,
    redis_con: MultiplexedConnection,
    secrets: Secrets,
) -> Router {
    let auth = AuthenticationService {
        db_pool: connection_pool,
        redis_con: Arc::new(Mutex::new(redis_con)),
        secrets,
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
}
