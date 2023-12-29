use std::time::Duration;

use auth::proto::auth::LoginRequest;
use tonic::{Code, Request};

use crate::helpers::spawn_app;

#[tokio::test]
async fn missing_field_in_login_request() {
    let app = spawn_app().await;

    // we sleep to make sure the server is up and running, here we should use the health check service on and essentially poll it until
    // the servcie is ready and then we can start to make the request
    tokio::time::sleep(Duration::from_millis(100)).await;

    let test_cases = vec![
        ("", "password", "missing username field"),
        ("user", "", "missing password field"),
    ];

    for (username, password, error_message) in test_cases {
        let response = app
            .login(Request::new(LoginRequest {
                username: username.into(),
                password: password.into(),
            }))
            .await;

        assert!(response
            .is_err_and(|x| x.code() == Code::InvalidArgument && x.message() == error_message))
    }
}
