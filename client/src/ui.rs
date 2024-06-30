use ratatui::{style::Stylize, widgets::Paragraph, Frame};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let area = f.size();
    f.render_widget(
        Paragraph::new("Hello, World! (press 'q' to quit)")
            .white()
            .on_magenta(),
        area,
    )
}
