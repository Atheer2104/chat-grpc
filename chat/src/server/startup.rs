use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use tonic::transport::{server::Router, Server};

use crate::proto::chat::{chatting_server::ChattingServer, FILE_DESCRIPTOR_SET};

use super::{auth_interceptor, ChatService};

pub fn build_server() -> Router {
    // how many values that the broadcast channel can keep
    let (sender, _) = broadcast::channel(100);
    let chat_service = ChatService {
        sender: Arc::new(Mutex::new(sender)),
    };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        // .add_service(health_service)
        .add_service(ChattingServer::with_interceptor(
            chat_service,
            auth_interceptor,
        ))
        .add_service(reflection_service)
}
