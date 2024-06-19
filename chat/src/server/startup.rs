use tonic::transport::{server::Router, Server};

use crate::proto::chat::FILE_DESCRIPTOR_SET;

pub fn build_server() -> Router {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        // .add_service(health_service)
        .add_service(reflection_service)
}
