use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub async fn action<'a>(app: &mut App<'a>, key_event: KeyEvent) {
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
            KeyCode::Char('w') => app.toggle_mode(),
            KeyCode::Char('j') | KeyCode::Down => app.home.select_next(),
            KeyCode::Char('k') | KeyCode::Up => app.home.select_previous(),
            KeyCode::Enter => app.home.select(),
            // for key events which we don't care about
            _ => {}
        },
        crate::app::AppMode::Write => match key_event.code {
            KeyCode::Esc => app.toggle_mode(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.exit()
                }
            }
            _ => {}
        },
    }
}
