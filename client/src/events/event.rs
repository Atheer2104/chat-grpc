use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppMode};

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
            KeyCode::Enter => app.home.login.submit(sender),
            KeyCode::Tab => app.home.login.focus_next(),
            KeyCode::BackTab => app.home.login.focus_prev(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.exit()
                }
            }
            // we are writing
            _ => app.home.login.handle_event_current_field(key_event),
        },
        crate::app::AppMode::Error => match key_event.code {
            KeyCode::Enter | KeyCode::Esc => {
                app.home.login.show_error_popup = false;
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
