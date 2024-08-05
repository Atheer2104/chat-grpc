use auth::authentication::{auth_client::AuthClient, LoginRequest, RegisterRequest, Token};
use tonic::{transport::Channel, Request};

const ADDRESS: &str = "http://[::1]:8000";

#[derive(Clone)]
pub struct AuthApi {
    client: AuthClient<Channel>,
}

impl AuthApi {
    pub async fn new() -> AuthApi {
        let client = AuthClient::connect(ADDRESS)
            .await
            .expect("failed to create client");

        Self { client }
    }

    pub async fn login(&mut self, login_request: LoginRequest) -> Result<Token, String> {
        let request = Request::new(login_request);

        let login_result = self.client.login(request).await;

        match login_result {
            Ok(res) => {
                // println!("token: {}", res.into_inner().access_token);
                Ok(res.into_inner())
            }
            Err(e) => Err(e.message().into()),
        }
    }

    pub async fn register(&mut self, register_request: RegisterRequest) -> Result<Token, String> {
        let request = Request::new(register_request);

        let register_result = self.client.register(request).await;

        match register_result {
            Ok(res) => {
                // println!("token: {}", res.into_inner().access_token);
                Ok(res.into_inner())
            }
            Err(e) => Err(e.message().into()),
        }
    }
}
