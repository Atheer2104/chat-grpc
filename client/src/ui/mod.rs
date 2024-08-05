mod utils;

pub use utils::*;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, AppView};

pub fn render(frame: &mut Frame, app: &mut App) {
    let full_original_framesize = frame.size();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(full_original_framesize);

    if app.view == AppView::Home {
        app.home.render(frame, main_layout[0])
    }

    app.footer.render(frame, main_layout[1], &app)

    // f.render_widget(Paragraph::new("Hello, World! (press 'q' to quit)"), area)
}
