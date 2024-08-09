use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, AppMode};

pub struct Footer {}

impl Default for Footer {
    fn default() -> Footer {
        Self {}
    }
}

impl Footer {
    pub fn new() -> Footer {
        Self::default()
    }

    pub fn render(&self, frame: &mut Frame, footer_area: Rect, app: &App) {
        let footer_text = match app.mode {
            AppMode::View => {
                vec![
                    Span::styled(" VIEW ", Style::default().bg(Color::Blue).bold()),
                    Span::styled(" Q or Ctrl + c: Quit.", Style::default()),
                    // Span::styled(" W: Write Mode.", Style::default()),
                    Span::styled(" Use ↓↑ to move", Style::default()),
                ]
            }
            AppMode::Write => {
                let mut text = vec![
                    Span::styled(" WRITE ", Style::default().bg(Color::Green).bold()),
                    // Span::styled(" Esc: go back to view mode. ", Style::default()),
                    Span::styled(" Esc : Go Back.", Style::default()),
                    Span::styled(" Enter : Submit.", Style::default()),
                    // Span::styled(" Ctrl: Quit.", Style::default()),
                ];

                if app.home.chat.chat_shown() {
                    text.push(Span::styled(" Use ↓↑ to Scroll. ", Style::default()))
                }

                text
            }
            AppMode::Error => {
                vec![
                    Span::styled(" ERROR ", Style::default().bg(Color::Red).bold()),
                    Span::styled(" Enter: to dismiss error.", Style::default()),
                    Span::styled(" Q or Ctrl + c: Quit.", Style::default()),
                ]
            }
        };

        let footer = Line::from(footer_text);

        frame.render_widget(Paragraph::new(footer).centered(), footer_area)
    }
}
