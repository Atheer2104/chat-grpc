use std::process::exit;

use anyhow::Result;
use chat::chat::ChatMessage;
use client::{
    api::{AuthApi, ChatApi},
    app::App,
    events::*,
    tui::Tui,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = Tui::default();
    terminal.initalize()?;

    let mut events = EventHandler::new(16);

    let mut app = App::new().await;

    let mut authapi = AuthApi::new().await;
    let mut chatapi = ChatApi::new(String::from("eyJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6InRlc3QyNkBnbWFpbCIsImV4cCI6IjE3MjMzOTk5NDYiLCJpYXQiOiIxNzIyNzk1MTQ2IiwiaXNzIjoiQ2hhdC1nUlBDIiwic3ViIjoiYXV0aCB0b2tlbiIsInVzZXJfaWQiOiIyNCIsInVzZXJuYW1lIjoidGVzdDI2In0.2API8Y6AVP4w4oHtNnjpWxgvU45PQUUnl6ak4iz0L5dRdWDWZcSI1CThUtHfxfuRfk1Fs8Gc8_ItjvSAQ2pHIQ")).await;

    // main program loop
    while !app.should_quit {
        terminal.draw(&mut app)?;

        match events.next().await? {
            Event::Key(key_event) => {
                action(&mut app, key_event, events.sender.clone()).await;
            }
            Event::Tick => {}
            Event::Mouse(_) => {}
            Event::Error => app.set_error_mode(),
            Event::Login => {
                let login_request = app.home.login.get_login_request();
                match authapi.login(login_request).await {
                    Ok(token) => {
                        app.set_access_token(token.access_token);
                        // println!("access token: {}", app.access_token)
                    }
                    Err(error_msg) => {
                        app.home.login.show_error_popup = true;
                        app.home.login.error_description = error_msg;
                        app.set_error_mode();
                    }
                };
            }
            Event::Register => {
                let register_request = app.home.register.get_register_request();
                match authapi.register(register_request).await {
                    Ok(token) => {
                        app.set_access_token(token.access_token);
                        // println!("access token: {}", app.access_token)
                    }
                    Err(error_msg) => {
                        app.home.register.show_error_popup = true;
                        app.home.register.error_description = error_msg;
                        app.set_error_mode();
                    }
                }
            }
            Event::Chat => {
                let message = app.home.chat.get_message();
                let chat_message = ChatMessage {
                    username: "atheer2104".into(),
                    message: message.into(),
                    timestamp: None,
                };

                chatapi.chat(chat_message, events.sender.clone()).await?;
            }
            Event::Message(message) => {
                println!("Recevied chat message: {:?}", message)
            }
        }
    }

    terminal.exit()?;
    Ok(())
}
