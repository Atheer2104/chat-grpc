use anyhow::Result;
use chat::chat::ChatMessage;
use client::{
    api::{AuthApi, ChatApi},
    app::App,
    events::*,
    tui::Tui,
};
use random_color::RandomColor;
use ratatui::style::Color;

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = Tui::default();
    terminal.initalize()?;

    let mut events = EventHandler::new(16);

    let mut app = App::new().await;

    let mut authapi = AuthApi::new().await;
    let mut chatapi: Option<ChatApi> = None;

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
                match authapi.login(login_request.clone()).await {
                    Ok(token) => {
                        // println!("access token: {}", token.access_token)
                        chatapi = Some(ChatApi::new(token.access_token).await);
                        app.username = login_request.username;
                        app.home.set_action_to_chat();
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
                match authapi.register(register_request.clone()).await {
                    Ok(token) => {
                        // println!("access token: {}", token.access_token)
                        chatapi = Some(ChatApi::new(token.access_token).await);
                        app.username = register_request.username;
                        app.home.set_action_to_chat();
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
                    username: app.username.clone(),
                    message: message.into(),
                    timestamp: None,
                };

                chatapi
                    .as_mut()
                    .unwrap()
                    .chat(chat_message, events.sender.clone())
                    .await?;
            }
            Event::Message(message) => {
                if !app
                    .home
                    .chat
                    .username_to_color
                    .contains_key(&message.username)
                {
                    // println!("adding new color");
                    let color_rgb = RandomColor::new()
                        // .luminosity(Luminosity::Bright)
                        .to_rgb_array();

                    app.home.chat.username_to_color.insert(
                        message.username.clone(),
                        Color::Rgb(color_rgb[0], color_rgb[1], color_rgb[2]),
                    );
                }

                app.home.chat.chat_messages.push(message);
                app.home.chat.reset_message_prompt_state();
            }
        }
    }

    terminal.exit()?;
    Ok(())
}
