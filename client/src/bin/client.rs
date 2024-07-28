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
                action(&mut app, key_event, events.sender.clone()).await;
            }
            Event::Tick => {}
            Event::Mouse(_) => {}
            Event::Error => app.set_error_mode(),
        }
    }

    terminal.exit()?;
    Ok(())
}
