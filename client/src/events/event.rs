use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, AppMode},
    components::Action,
};

use super::{Event, Sender};

pub async fn action<'a>(app: &mut App<'a>, key_event: KeyEvent, sender: Sender) {
    match app.mode {
        crate::app::AppMode::View => match key_event.code {
            KeyCode::Char('q') => {
                app.exit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.exit()
                }
            }
            // KeyCode::Char('w') => app.toggle_mode(),
            KeyCode::Char('j') | KeyCode::Down => app.home.select_next(),
            KeyCode::Char('k') | KeyCode::Up => app.home.select_previous(),
            KeyCode::Enter => {
                app.home.select();
                app.toggle_mode()
            }
            // for key events which we don't care about
            _ => {}
        },
        crate::app::AppMode::Write => match key_event.code {
            // KeyCode::Esc => app.toggle_mode(),
            KeyCode::Enter => {
                if let Some(action) = app.home.selected_action() {
                    match action {
                        Action::Login => {
                            app.home.login.submit(sender);
                            match app.home.login.is_finished() {
                                true => {
                                    let login_request = app.home.login.get_login_request();
                                    match app.authapi.login(login_request).await {
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
                                false => {}
                            }
                        }
                        Action::Register => app.home.register.submit(sender),
                        Action::Chat => todo!(),
                    }
                }
            }

            KeyCode::Tab => {
                if let Some(action) = app.home.selected_action() {
                    match action {
                        Action::Login => app.home.login.focus_next(),
                        Action::Register => app.home.register.focus_next(),
                        Action::Chat => todo!(),
                    }
                }
            }
            KeyCode::BackTab => {
                if let Some(action) = app.home.selected_action() {
                    match action {
                        Action::Login => app.home.login.focus_prev(),
                        Action::Register => app.home.register.focus_prev(),
                        Action::Chat => todo!(),
                    }
                }
            }

            KeyCode::Esc => {
                app.home.reset_action();
                app.toggle_mode();
            }
            // we are writing
            _ => {
                if let Some(action) = app.home.selected_action() {
                    match action {
                        Action::Login => app.home.login.handle_event_current_field(key_event),
                        Action::Register => app.home.register.handle_event_current_field(key_event),
                        Action::Chat => todo!(),
                    }
                }
            }
        },
        crate::app::AppMode::Error => match key_event.code {
            KeyCode::Enter | KeyCode::Esc => {
                if let Some(action) = app.home.selected_action() {
                    match action {
                        Action::Login => app.home.login.show_error_popup = false,
                        Action::Register => app.home.register.show_error_popup = false,
                        Action::Chat => todo!(),
                    }
                }
                app.mode = AppMode::Write;
            }
            KeyCode::Char('q') => {
                app.exit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.exit()
                }
            }
            _ => {}
        },
    }
}
