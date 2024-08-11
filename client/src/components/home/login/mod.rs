use auth::authentication::LoginRequest;
use tui_popup::Popup;

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Padding, Paragraph},
    Frame,
};
use tui_prompts::{Prompt, State, TextPrompt, TextRenderStyle, TextState};

use crate::{
    events::{Event, Sender},
    ui::centered_rect,
};

use super::{validate_password, validate_username};

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
    pub show_error_popup: bool,
    pub error_description: String,
}

impl<'a> Default for Login<'a> {
    fn default() -> Login<'a> {
        Self {
            show_login: false,
            current_field: Field::default(),
            username_state: TextState::default(),
            password_state: TextState::default(),
            show_error_popup: false,
            error_description: String::from(""),
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

    pub fn show_login_error_popup(&self) -> bool {
        self.show_error_popup
    }

    pub fn is_finished(&self) -> bool {
        self.username_state.is_finished() && self.password_state.is_finished()
    }

    pub fn get_login_request(&self) -> LoginRequest {
        let username = self.username_state.value();
        let password = self.password_state.value();
        LoginRequest {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn reset_textfields_state(&mut self) {
        self.username_state = TextState::default();
        self.password_state = TextState::default();
    }

    pub fn focus_next(&mut self) {
        self.current_state().blur();
        if let Some(field) = self.next_field() {
            self.current_field = field;
        }
        self.current_state().focus();
    }

    pub fn focus_prev(&mut self) {
        self.current_state().blur();
        if let Some(field) = self.prev_field() {
            self.current_field = field
        }
        self.current_state().focus();
    }

    fn focus_current_field(&mut self) {
        self.current_state().focus();
    }

    pub fn submit(&mut self, sender: Sender) {
        // have to validate the value here then mark it as complete

        let validation_result = match self.current_field {
            Field::Username => validate_username(self.current_state().value()),
            Field::Password => validate_password(self.current_state().value()),
        };

        match validation_result {
            Ok(_) => {
                self.show_error_popup = false;
                self.current_state().complete();

                if self.current_state().is_finished() && !self.is_finished() {
                    self.focus_next()
                } else {
                    // everything is complete
                    // println!("all done");
                    // println!(
                    //     "username: {}, password: {}",
                    //     self.username_state.value(),
                    //     self.password_state.value()
                    // )
                }
            }
            Err(e) => {
                self.show_error_popup = true;
                self.error_description = e;
                self.current_state().abort();
                self.current_state().blur();
                sender.send(Event::Error).unwrap();
            }
        }
    }

    pub fn handle_event_current_field(&mut self, key_event: KeyEvent) {
        let state = self.current_state();
        state.handle_key_event(key_event);
    }

    fn next_field(&mut self) -> Option<Field> {
        if !self.current_state().status().is_aborted() {
            return match self.current_field {
                Field::Username => Some(Field::Password),
                Field::Password => Some(Field::Username),
            };
        }

        None
    }

    fn prev_field(&mut self) -> Option<Field> {
        if !self.current_state().status().is_aborted() {
            return match self.current_field {
                Field::Username => Some(Field::Password),
                Field::Password => Some(Field::Username),
            };
        }

        None
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

        if self.show_error_popup {
            let error_popup = Popup::new(self.error_description.as_str())
                .title("Login Error")
                .style(Style::default().on_red());

            frame.render_widget(&error_popup, area)
        } else {
            // when we have an error we enter error mode which will unfoucus the current field thus we have to focus back the current field
            self.focus_current_field()
        }
    }
}
