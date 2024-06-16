use auth::proto::auth::{LoginRequest, RegisterRequest};
use rand::{thread_rng, Rng};
use tonic_types::StatusExt;

use super::{sleep, spawn_app, Code, Request};

#[tokio::test]
async fn missing_field_in_login_request() {
    let app = spawn_app().await;
    let mut rng = thread_rng();
    sleep(rng.gen_range(100..200)).await;

    let test_cases = vec![("", "password"), ("user", "")];

    for (username, password) in test_cases {
        let response = app
            .login(Request::new(LoginRequest {
                username: username.into(),
                password: password.into(),
            }))
            .await;

        let error = response.expect_err("The response was not a error");

        println!("ERROR (MISSING_FIELDS): {:?}", error);

        // check the code for status is a invalid argument
        assert!(error.code() == Code::InvalidArgument);

        // check that status contains bad request violations
        assert!(error.get_error_details().has_bad_request_violations());
    }
}

#[tokio::test]
async fn login_as_non_existing_user() {
    let app = spawn_app().await;
    let mut rng = thread_rng();
    sleep(rng.gen_range(100..200)).await;

    let response = app
        .login(Request::new(LoginRequest {
            username: "ABCDEFG".into(),
            password: "123456789".into(),
        }))
        .await;

    let error = response.expect_err("The response was not a error");

    println!("ERROR (NON_EXISTING_USER): {:?}", error);

    assert!(error.code() == Code::Unauthenticated);
}

#[tokio::test]
async fn login_as_a_registered_user() {
    let app = spawn_app().await;
    let mut rng = thread_rng();
    sleep(rng.gen_range(100..200)).await;

    let register_response = app
        .register(Request::new(RegisterRequest {
            firstname: "atheer".into(),
            lastname: "ABC".into(),
            username: "atheer2104".into(),
            email: "atheer@gmail.com".into(),
            password: "strong password".into(),
        }))
        .await;

    println!("REGISTER RESPONSE: {:?}", register_response);

    assert!(register_response.is_ok());

    let login_request = app
        .login(Request::new(LoginRequest {
            username: "atheer2104".into(),
            password: "strong password".into(),
        }))
        .await;

    println!("LOGIN REQUEST: {:?}", login_request);

    assert!(login_request.is_ok());
}

#[tokio::test]
async fn login_as_a_registered_user_check_token() {
    let app = spawn_app().await;
    let mut rng = thread_rng();
    sleep(rng.gen_range(100..200)).await;

    let username = String::from("atheer");

    let register_response = app
        .register(Request::new(RegisterRequest {
            firstname: "atheer".into(),
            lastname: "ABC".into(),
            username: username.clone(),
            email: "atheer@gmail.com".into(),
            password: "strong password".into(),
        }))
        .await;

    println!("REGISTER RESPONSE: {:?}", register_response);

    assert!(register_response.is_ok());

    let login_request = app
        .login(Request::new(LoginRequest {
            username: username.clone(),
            password: "strong password".into(),
        }))
        .await;

    let login_request_token = login_request
        .expect("there was en error in the login request")
        .into_inner()
        .access_token;

    let user_id = sqlx::query!(
        r#"SELECT user_id FROM account WHERE username = $1 "#,
        username
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed to fetch from db")
    .user_id;

    let token_from_db = sqlx::query!(
        r#"SELECT auth_token FROM auth_tokens WHERE user_id = $1 "#,
        user_id
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("failed to fetch from db")
    .auth_token;

    assert_eq!(token_from_db, login_request_token)
}
