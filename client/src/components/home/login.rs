use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, BorderType, Clear, Padding},
    Frame,
};
use tui_prompts::{Prompt, TextPrompt, TextRenderStyle, TextState};

use crate::ui::centered_rect;

pub struct Login<'a> {
    show_login: bool,
    username_state: TextState<'a>,
    password_state: TextState<'a>,
}

impl<'a> Default for Login<'a> {
    fn default() -> Login<'a> {
        Self {
            show_login: false,
            username_state: TextState::default(),
            password_state: TextState::default(),
        }
    }
}

impl<'a> Login<'a> {
    pub fn new() -> Login<'a> {
        Self::default()
    }

    pub fn toggle_login(&mut self) {
        self.show_login = !self.show_login
    }

    pub fn show_login(&self) -> bool {
        self.show_login
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let login_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(5))
            .title("Login Form".bold().into_centered_line());

        let block_area = centered_rect(40, 10, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .split(login_block.inner(block_area));

        //clearing our the area where the pop will be on top
        frame.render_widget(Clear, block_area);
        frame.render_widget(login_block, block_area);

        TextPrompt::from("Username").draw(frame, layout[1], &mut self.username_state);

        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, layout[2], &mut self.password_state);
    }
}
