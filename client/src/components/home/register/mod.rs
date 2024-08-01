use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Padding, Paragraph},
    Frame,
};
use tui_popup::Popup;
use tui_prompts::{Prompt, State, TextPrompt, TextRenderStyle, TextState};

use crate::{
    events::{Event, Sender},
    ui::centered_rect,
};

use super::{parse_email, validate_name, validate_password, validate_username};

enum Field {
    Firstname,
    Lastname,
    Username,
    Email,
    Password,
}

pub struct Register<'a> {
    show_register: bool,
    current_field: Field,
    firstname_state: TextState<'a>,
    lastname_state: TextState<'a>,
    username_state: TextState<'a>,
    email_state: TextState<'a>,
    password_state: TextState<'a>,
    pub show_error_popup: bool,
    error_description: String,
}

impl<'a> Default for Register<'a> {
    fn default() -> Register<'a> {
        Self {
            show_register: false,
            current_field: Field::Firstname,
            firstname_state: TextState::default(),
            lastname_state: TextState::default(),
            username_state: TextState::default(),
            email_state: TextState::default(),
            password_state: TextState::default(),
            show_error_popup: false,
            error_description: "".into(),
        }
    }
}

impl<'a> Register<'a> {
    pub fn new() -> Register<'a> {
        Self::default()
    }

    pub fn toggle_register(&mut self) {
        self.show_register = !self.show_register
    }

    fn is_finished(&self) -> bool {
        self.firstname_state.is_finished()
            && self.lastname_state.is_finished()
            && self.username_state.is_finished()
            && self.email_state.is_finished()
            && self.password_state.is_finished()
    }

    fn focus_current_field(&mut self) {
        self.current_state().focus();
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
            self.current_field = field;
        }
        self.current_state().focus();
    }

    pub fn submit(&mut self, sender: Sender) {
        let validation_result = match self.current_field {
            Field::Firstname => validate_name(self.current_state().value(), "Firstname"),
            Field::Lastname => validate_name(self.current_state().value(), "Lastname"),
            Field::Username => validate_username(self.current_state().value()),
            Field::Email => parse_email(self.current_state().value()),
            Field::Password => validate_password(self.current_state().value()),
        };

        match validation_result {
            Ok(_) => {
                self.show_error_popup = false;
                self.current_state().complete();

                if self.current_state().is_finished() && !self.is_finished() {
                    self.focus_next();
                } else {
                    // complete
                    // println!("alles god")
                }
            }
            Err(e) => {
                self.show_error_popup = true;
                self.error_description = e;
                self.current_state().abort();
                self.current_state().blur();
                sender.send(Event::Error);
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
                Field::Firstname => Some(Field::Lastname),
                Field::Lastname => Some(Field::Username),
                Field::Username => Some(Field::Email),
                Field::Email => Some(Field::Password),
                Field::Password => Some(Field::Firstname),
            };
        }

        None
    }

    fn prev_field(&mut self) -> Option<Field> {
        if !self.current_state().status().is_aborted() {
            return match self.current_field {
                Field::Firstname => Some(Field::Password),
                Field::Lastname => Some(Field::Firstname),
                Field::Username => Some(Field::Lastname),
                Field::Email => Some(Field::Username),
                Field::Password => Some(Field::Email),
            };
        }

        None
    }

    fn current_state(&mut self) -> &mut TextState<'a> {
        match self.current_field {
            Field::Firstname => &mut self.firstname_state,
            Field::Lastname => &mut self.lastname_state,
            Field::Username => &mut self.username_state,
            Field::Email => &mut self.email_state,
            Field::Password => &mut self.password_state,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let register_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2))
            .title("Register Form".bold().into_centered_line());

        let block_area = centered_rect(45, 45, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                // firstname
                Constraint::Length(1),
                Constraint::Length(3),
                // lastname
                Constraint::Length(1),
                Constraint::Length(3),
                // username
                Constraint::Length(1),
                Constraint::Length(4),
                //email
                Constraint::Length(2),
                // password
                Constraint::Length(1),
                Constraint::Length(2),
            ])
            .split(register_block.inner(block_area));

        frame.render_widget(Clear, block_area);
        frame.render_widget(register_block, block_area);

        TextPrompt::from("Firstname").draw(frame, layout[1], &mut self.firstname_state);

        let name_helper_text = vec![
            Line::from(Span::styled("Maximum of 255 character", Style::default())),
            Line::from(Span::styled(
                "Cannot contains numbers",
                Style::default().red(),
            )),
        ];

        let name_helper_text = Paragraph::new(name_helper_text);
        frame.render_widget(name_helper_text.clone(), layout[2]);

        TextPrompt::from("Lastname").draw(frame, layout[3], &mut self.lastname_state);

        frame.render_widget(name_helper_text, layout[4]);

        TextPrompt::from("Username").draw(frame, layout[5], &mut self.username_state);

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
        frame.render_widget(username_helper_paragraph, layout[6]);

        TextPrompt::from("Email").draw(frame, layout[7], &mut self.email_state);

        TextPrompt::from("Password")
            .with_render_style(TextRenderStyle::Password)
            .draw(frame, layout[8], &mut self.password_state);

        let password_helper_text = vec![
            Line::from(Span::styled("Minimum of 8 character", Style::default())),
            Line::from(Span::styled("Maximum of 255 character", Style::default())),
        ];
        let password_helper_paragraph = Paragraph::new(password_helper_text);
        frame.render_widget(password_helper_paragraph, layout[9]);

        if self.show_error_popup {
            // popup error goes here
            let error_popup = Popup::new(self.error_description.as_str())
                .title("Register Error")
                .style(Style::default().on_red());

            frame.render_widget(&error_popup, area)
        } else {
            self.focus_current_field()
        }
    }
}
