use std::net::SocketAddr;

use auth::{
    configuration::{get_configuration, DatabaseSettings},
    logging::{get_subscriber, init_subscriber},
    server::run_server,
};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

// this makes sure that we only initialize tracing only once for, this is because the test runs in parallel with would mean that
// this would be initalized for each test which is not what one wants, note this is lazy ie will be initalized when first used
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "chat-grpc-auth-TEST".to_string();
    // checking if we want to output logs we do to stdout othwerise we output to sink which will not ouput anything at all
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(config.connection_string_no_db().expose_secret())
        .await
        .expect("Failed to connect to postgresql");
    connection
        .execute(&*format!(r#"CREATE DATABASE {}"#, config.database_name))
        .await
        .expect("Failed to create database");

    // running our migrations
    let connection_pool = PgPool::connect(config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to postgresql");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the DB");

    connection_pool
}

pub struct App {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> App {
    // we force evaluate TRACING
    Lazy::force(&TRACING);

    let address: SocketAddr = format!("[::1]:0")
        .parse()
        .expect("Failed to bind to a random port");

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    run_server(connection_pool.clone(), address)
        .await
        .expect("Failed to create server");

    App {
        address: address.to_string(),
        db_pool: connection_pool,
    }
}
