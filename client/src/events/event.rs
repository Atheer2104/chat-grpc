use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub async fn action(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => {
            app.exit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.exit()
            }
        }
        // for key events which we don't care about
        _ => {}
    }
}
