use std::process::exit;

use anyhow::Result;
use client::{app::App, events::*, tui::Tui};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = Tui::default();
    terminal.initalize()?;

    let mut events = EventHandler::new(16);

    let mut app = App::new();

    // main program loop
    while !app.should_quit {
        terminal.draw(&mut app)?;

        match events.next().await? {
            Event::Key(key_event) => {
                action(&mut app, key_event).await;
            }
        }
    }

    terminal.exit()?;
    // ! I know this is how one should not to do it, but the problem is that the task spawned in event handler is still alive even after the main
    // ! program has finished, thus is does not actually terminate.
    exit(0);
}
