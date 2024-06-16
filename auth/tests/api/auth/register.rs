use std::collections::BTreeMap;

use auth::proto::auth::RegisterRequest;
use hmac::{digest::KeyInit, Hmac};
use jwt::VerifyWithKey;
use rand::{thread_rng, Rng};
use redis::AsyncCommands;
use secrecy::ExposeSecret;
use sha2::Sha512;

use super::{sleep, spawn_app, Request};

#[tokio::test]
async fn register_user_dont_check_token() {
    let app = spawn_app().await;
    let mut rng = thread_rng();
    sleep(rng.gen_range(100..200)).await;

    let response = app
        .register(Request::new(RegisterRequest {
            firstname: "atheer".into(),
            lastname: "ABC".into(),
            username: "atheer2104".into(),
            email: "atheer@gmail.com".into(),
            password: "strong password".into(),
        }))
        .await;

    assert!(response.is_ok());

    let saved = sqlx::query!(r#"SELECT * FROM account"#)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch from db");

    assert_eq!(saved.firstname, "atheer");
    assert_eq!(saved.lastname, "ABC");
    assert_eq!(saved.username, "atheer2104");
    assert_eq!(saved.email, "atheer@gmail.com");
}

#[tokio::test]
async fn register_user_check_token() {
    let app = spawn_app().await;
    let mut rng = thread_rng();
    sleep(rng.gen_range(100..200)).await;

    let username = String::from("atheer21");

    let response = app
        .register(Request::new(RegisterRequest {
            firstname: "atheer".into(),
            lastname: "abc".into(),
            username: username.clone(),
            email: "atheer21@gmail.com".into(),
            password: "secret secret".into(),
        }))
        .await;

    let auth_token_response = response
        .expect("no auth token found creating user")
        .into_inner();

    println!("auth token: {:?}", auth_token_response);

    let saved = sqlx::query!(
        r#"SELECT user_id FROM account WHERE username = $1"#,
        username,
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed to fetch from db");

    println!("fetched user_id: {}", saved.user_id);

    let mut redis_con = app.redis_con.lock().await;

    let auth_token_redis: String = match redis_con.get(saved.user_id).await {
        Ok(token) => token,
        Err(_) => panic!("couldn't get user_id from redis"),
    };

    println!("auth token redis: {}", auth_token_redis);

    assert_eq!(auth_token_redis, auth_token_response.access_token);

    let key: Hmac<Sha512> = Hmac::new_from_slice(app.secrets.jwt_secret.expose_secret().as_bytes())
        .expect("failed to create hmac");

    let claims: BTreeMap<String, String> = auth_token_response
        .access_token
        .verify_with_key(&key)
        .expect("failed to verify auth token");

    println!("claims: {:?}", claims);

    assert_eq!(claims["iss"], "Chat-gRPC");
}
