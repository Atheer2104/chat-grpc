use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Wrap},
    Frame,
};
use tui_prompts::{FocusState, Prompt, State, TextPrompt, TextRenderStyle, TextState};

use crate::ui::centered_rect;

#[derive(Default)]
enum Field {
    #[default]
    Username,
    Password,
}

pub struct Login<'a> {
    show_login: bool,
    current_field: Field,
    username_state: TextState<'a>,
    password_state: TextState<'a>,
}

impl<'a> Default for Login<'a> {
    fn default() -> Login<'a> {
        Self {
            show_login: false,
            current_field: Field::default(),
            username_state: TextState::default().with_focus(FocusState::Focused),
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

    fn is_finished(&self) -> bool {
        self.username_state.is_finished() && self.password_state.is_finished()
    }

    pub fn focus_next(&mut self) {
        self.current_state().blur();
        self.current_field = self.next_field();
        self.current_state().focus();
    }

    pub fn focus_prev(&mut self) {
        self.current_state().blur();
        self.current_field = self.prev_field();
        self.current_state().focus();
    }

    pub fn submit(&mut self) {
        // have to validate the value here then mark it as complete
        self.current_state().complete();
        if self.current_state().is_finished() && !self.is_finished() {
            self.current_state().blur();
            self.current_field = self.next_field();
            self.current_state().focus();
        } else {
            // everything is complete
            // println!(
            //     "username: {}, password: {}",
            //     self.username_state.value(),
            //     self.password_state.value()
            // )
        }
    }

    pub fn handle_event_current_field(&mut self, key_event: KeyEvent) {
        let state = self.current_state();
        state.handle_key_event(key_event);
    }

    fn next_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Username,
        }
    }

    fn prev_field(&mut self) -> Field {
        match self.current_field {
            Field::Username => Field::Password,
            Field::Password => Field::Username,
        }
    }

    fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Username => &mut self.username_state,
            Field::Password => &mut self.password_state,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let login_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
            .title("Login Form".bold().into_centered_line());

        let block_area = centered_rect(45, 25, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Length(1),
                Constraint::Length(4),
                Constraint::Length(1),
                Constraint::Length(2),
            ])
            .split(login_block.inner(block_area));

        //clearing our the area where the pop will be on top
        frame.render_widget(Clear, block_area);
        frame.render_widget(login_block, block_area);

        TextPrompt::from("Username").draw(frame, layout[1], &mut self.username_state);

        let username_helper_text = vec![
            Line::from(Span::styled("Maximum of 255 character", Style::default())),
            Line::from(Span::styled(
                "Following charcters are forbidden",
                Style::default(),
            )),
            Line::from(Span::styled(
                format!(
                    "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                    '/', '(', ')', '"', '<', '>', '\\', '{', '}'
                ),
                Style::default().red(),
            )),
        ];

        let username_helper_paragraph = Paragraph::new(username_helper_text);
        frame.render_widget(username_helper_paragraph, layout[2]);

        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, layout[3], &mut self.password_state);

        let password_helper_text = vec![
            Line::from(Span::styled("Minimum of 8 character", Style::default())),
            Line::from(Span::styled("Maximum of 255 character", Style::default())),
        ];
        let password_helper_paragraph = Paragraph::new(password_helper_text);
        frame.render_widget(password_helper_paragraph, layout[4]);
    }
}
