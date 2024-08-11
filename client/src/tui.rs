use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::io::{stdout, Stdout};

use crate::{app::App, ui::render};

/// A type alias for the terminal type used in this application
pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

impl Tui {
    pub fn new() -> Tui {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).expect("Failed to create terminal");
        Self { terminal }
    }

    pub fn initalize(&mut self) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| render(frame, app))?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        disable_raw_mode()?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
