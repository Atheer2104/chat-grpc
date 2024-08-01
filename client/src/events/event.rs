use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, AppMode},
    components::Action,
};

use super::Sender;

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
                        Action::Login => app.home.login.submit(sender),
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

            KeyCode::Esc => app.exit(),
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
