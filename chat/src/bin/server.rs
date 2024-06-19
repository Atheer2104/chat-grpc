use std::net::SocketAddr;

use chat::{
    configuration::get_configuration,
    logging::{get_subscriber, init_subscriber},
    server::build_server,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let subscriber = get_subscriber(
        "Chat-gRPC Chat-service".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    tracing::info!("Reading configuration");

    let configuration = get_configuration().expect("Failed to read config file");

    tracing::info!("Building gRPC Server");

    let server = build_server();

    tracing::info!("Succesfully built gRPC Server");

    let address: SocketAddr = format!("[::1]:{}", configuration.application_port).parse()?;

    server.serve(address).await?;

    tracing::info!("Successfully served Server");

    Ok(())
}
