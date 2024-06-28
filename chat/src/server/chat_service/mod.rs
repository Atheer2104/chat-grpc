use std::{pin::Pin, sync::Arc};

use tokio::sync::{broadcast, Mutex};

use tonic::{Request, Response, Status};

use crate::proto::chat::{chatting_server::Chatting, ChatMessage};

use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
#[derive(Debug)]
pub struct ChatService {
    pub sender: Arc<Mutex<broadcast::Sender<ChatMessage>>>,
}

#[tonic::async_trait]
impl Chatting for ChatService {
    type chatStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send + 'static>>;

    #[tracing::instrument(
        name = "Chat service"
        skip(self, request)
    )]
    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::chatStream>, Status> {
        let mut incoming_data = request.into_inner();

        let receiver = self.sender.lock().await.subscribe();
        let mut outbound_messages = BroadcastStream::new(receiver);

        let sender = self.sender.clone();

        tokio::spawn(async move {
            while let Some(message) = incoming_data.next().await {
                let message = match message {
                    Ok(message) => message,
                    Err(e) => {
                        tracing::error!("Error receiving message: {:?}", e);
                        break;
                    }
                };

                let _ = sender.lock().await.send(message);
            }
        });

        // Map broadcast stream to tonic stream
        let output = async_stream::stream! {
            while let Some(result) = outbound_messages.next().await {
                match result {
                    Ok(message) => yield Ok(message),
                    Err(e) => {
                        eprintln!("Error in outbound stream: {:?}", e);
                        yield Err(Status::internal("Error in outbound stream"));
                    }
                }
            }
        };

        Ok(Response::new(Box::pin(output) as Self::chatStream))
    }
}
