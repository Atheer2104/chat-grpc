use anyhow::Result;
use chat::chat::{chatting_client::ChattingClient, ChatMessage};
use tonic::{
    body::BoxBody,
    metadata::MetadataValue,
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status,
};

const ADDRESS: &str = "http://[::1]:8001";

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ChatApi {
    client: ChattingClient<InterceptedService<Channel, MyInterceptor>>,
}

impl ChatApi {
    pub async fn new(access_token: String) -> Self {
        let channel = Channel::from_static(ADDRESS)
            .connect()
            .await
            .expect("Failed to connect to chat service");

        // let token: MetadataValue<_> = format!("Bearer {}", access_token)
        //     .parse()
        //     .expect("Failed to create access token");

        let client: ChattingClient<InterceptedService<Channel, MyInterceptor>> =
            ChattingClient::with_interceptor(channel, MyInterceptor { access_token });

        Self { client }
    }

    pub async fn chat(&mut self, chat_message: ChatMessage) -> Result<()> {
        let outbound = async_stream::stream! {
            yield chat_message
        };

        let response = self.client.chat(Request::new(outbound)).await?;
        let mut inbound = response.into_inner();

        while let Some(message) = inbound.message().await? {
            println!("Received Message: {:?}", message);
        }

        Ok(())
    }
}
