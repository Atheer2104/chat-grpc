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

    let configuartion = get_configuration().expect("Failed to read config file");
    let connection_pool =
        PgPool::connect(configuartion.database.connection_string().expose_secret())
            .await
            .expect("failed to connect to postgres");

    let address: SocketAddr = format!("[::1]:{}", configuartion.application_port).parse()?;

    //println!("Address {:?}", address);

    build_server(connection_pool).serve(address).await?;

    // .serve(address).await?;

    Ok(())
}
