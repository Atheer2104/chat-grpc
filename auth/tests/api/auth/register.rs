use auth::proto::auth::RegisterRequest;
use rand::{thread_rng, Rng};
use tokio::task::spawn_blocking;

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
