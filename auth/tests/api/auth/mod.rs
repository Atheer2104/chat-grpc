mod login;
mod register;

use std::time::Duration;
use tonic::{Code, Request};

use crate::helpers::spawn_app;

pub async fn sleep(duration: u64) {
    // we sleep to make sure the server is up and running, here we should use the health check service on and essentially poll it until
    // the servcie is ready and then we can start to make the request
    tokio::time::sleep(Duration::from_millis(duration)).await;
}
