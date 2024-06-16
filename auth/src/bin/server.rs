use anyhow::anyhow;
use auth::configuration::get_configuration;
use auth::logging::{get_subscriber, init_subscriber};
use auth::secrets::get_secrets;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

use auth::server::build_server;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber("chat-grpc-auth".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    tracing::info!("Reading configuration");

    let configuration = get_configuration().expect("Failed to read config file");

    tracing::info!("Reading secrets");
    let secrets = get_secrets().expect("Failed to read secrets file");

    tracing::info!("Connecting to PostgreSQL database");

    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("failed to connect to postgres");

    tracing::info!("Successfully connected to PostgreSQL database");

    tracing::info!("Creating redis client");

    let redis_client = redis::Client::open(configuration.redis_uri.expose_secret().to_owned())?;
    let mut redis_con = match redis_client.get_multiplexed_async_connection().await {
        Ok(con) => con,
        Err(_) => return Err(anyhow!("couldn't get a redis connection")),
    };

    tracing::info!("Successfully created redis client");

    //println!("Address {:?}", address);

    tracing::info!("Building gRPC Server");

    let server = build_server(connection_pool, redis_con, secrets);

    tracing::info!("Succesfully built gRPC Server");

    let address: SocketAddr = format!("[::1]:{}", configuration.application_port).parse()?;

    server.serve(address).await?;

    tracing::info!("Successfully served Server");

    Ok(())
}
