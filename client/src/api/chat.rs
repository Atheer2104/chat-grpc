use std::sync::Arc;

use anyhow::Result;
use chat::chat::{chatting_client::ChattingClient, ChatMessage};
use ratatui::symbols::block::HALF;
use std::sync::Mutex;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use tonic::{
    metadata::MetadataValue,
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status,
};

use crate::events::{Event, Sender};

const ADDRESS: &str = "http://[::1]:8001";

struct MyInterceptor {
    access_token: String,
}

impl Interceptor for MyInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, Status> {
        let token: MetadataValue<_> = format!("Bearer {}", self.access_token)
            .parse()
            .expect("Failed to create access token");
        request.metadata_mut().insert("authorization", token);
        Ok(request)
    }
}

pub struct ChatApi {
    pub sender: UnboundedSender<ChatMessage>,
}

impl ChatApi {
    pub async fn new(access_token: String, event_sender: Sender) -> Self {
        let channel = Channel::from_static(ADDRESS)
            .connect()
            .await
            .expect("Failed to connect to chat service");

        let mut client: ChattingClient<InterceptedService<Channel, MyInterceptor>> =
            ChattingClient::with_interceptor(channel, MyInterceptor { access_token });

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let mut messages_to_send = UnboundedReceiverStream::new(rx);

        tokio::spawn(async move {
            let outbound = async_stream::stream! {
                while let Some(message) = messages_to_send.next().await {
                    yield message
                }
            };

            let response = client
                .chat(Request::new(outbound))
                .await
                .expect("Failed to get message");

            let mut inbound = response.into_inner();

            while let Some(message) = inbound.message().await.expect("Failed to read") {
                let _ = event_sender.send(Event::Message(message));
            }
        });

        Self { sender: tx }
    }

    pub async fn chat(&mut self, chat_message: ChatMessage) {
        let _ = self.sender.send(chat_message);
    }
}
