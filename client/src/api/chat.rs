use std::sync::Arc;

use anyhow::Result;
use chat::chat::{chatting_client::ChattingClient, ChatMessage};
use ratatui::symbols::block::HALF;
use std::sync::Mutex;
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
    client: ChattingClient<InterceptedService<Channel, MyInterceptor>>,
    spawn_inbound_stream_task: bool,
}

impl ChatApi {
    pub async fn new(access_token: String) -> Self {
        let channel = Channel::from_static(ADDRESS)
            .connect()
            .await
            .expect("Failed to connect to chat service");

        let client: ChattingClient<InterceptedService<Channel, MyInterceptor>> =
            ChattingClient::with_interceptor(channel, MyInterceptor { access_token });

        Self {
            client,
            spawn_inbound_stream_task: false,
        }
    }

    pub async fn chat(&mut self, chat_message: ChatMessage, sender: Sender) -> Result<()> {
        let outbound = async_stream::stream! {
            yield chat_message
        };

        let response = self.client.chat(Request::new(outbound)).await?;
        let mut inbound = response.into_inner();

        if !self.spawn_inbound_stream_task {
            self.spawn_inbound_stream_task = true;
            tokio::spawn(async move {
                while let Some(message) = inbound.message().await.expect("Failed to read") {
                    // println!("Received Message: {:?}", message);
                    let _ = sender.send(Event::Message(message));
                }
            });
        }

        Ok(())
    }
}
