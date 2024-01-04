use auth::configuration::get_configuration;
use auth::logging::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::error::Error;
use std::net::SocketAddr;

use auth::server::build_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = get_subscriber("chat-grpc-auth".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    tracing::info!("Reading configuration");

    let configuartion = get_configuration().expect("Failed to read config file");

    tracing::info!("Connecting to PostgreSQL database");

    let connection_pool =
        PgPool::connect(configuartion.database.connection_string().expose_secret())
            .await
            .expect("failed to connect to postgres");

    tracing::info!("Successfully connected to PostgreSQL database");

    //println!("Address {:?}", address);

    tracing::info!("Building gRPC Server");

    let server = build_server(connection_pool);

    tracing::info!("Succesfully built gRPC Server");

    let address: SocketAddr = format!("[::1]:{}", configuartion.application_port).parse()?;

    server.serve(address).await?;

    tracing::info!("Successfully served Server");

    Ok(())
}
