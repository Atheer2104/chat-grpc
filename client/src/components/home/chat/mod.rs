use std::collections::HashMap;

use chat::chat::ChatMessage;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame,
};
use tui_prompts::{FocusState, Prompt, State, TextPrompt, TextState};

use crate::events::{Event, Sender};

pub struct Chat<'a> {
    show_chat: bool,
    message_prompt_state: TextState<'a>,
    pub vertical_scroll: u16,
    pub username_to_color: HashMap<String, Color>,
    pub chat_messages: Vec<ChatMessage>,
}

impl<'a> Default for Chat<'a> {
    fn default() -> Chat<'a> {
        Self {
            show_chat: false,
            message_prompt_state: TextState::default().with_focus(FocusState::Focused),
            vertical_scroll: 0,
            username_to_color: HashMap::new(),
            chat_messages: Vec::new(),
        }
    }
}

impl<'a> Chat<'a> {
    pub fn new() -> Chat<'a> {
        Self::default()
    }

    pub fn chat_shown(&self) -> bool {
        self.show_chat
    }

    pub fn toggle_chat(&mut self) {
        self.show_chat = !self.show_chat
    }

    pub fn handle_event(&mut self, key_event: KeyEvent) {
        self.message_prompt_state.handle_key_event(key_event)
    }

    pub fn get_message(&self) -> &str {
        self.message_prompt_state.value()
    }

    pub fn reset_message_prompt_state(&mut self) {
        self.message_prompt_state = TextState::default().with_focus(FocusState::Focused)
    }

    pub fn handle_submit(&self, sender: Sender) {
        let _ = sender.send(Event::Chat);

        // println!("message to send: {}", message);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(2),
                Constraint::Min(1),
                Constraint::Percentage(15),
            ])
            .split(area);

        //clearing our the area where the pop will be on top
        frame.render_widget(Clear, area);

        TextPrompt::from("Message Prompt")
            .with_block(Block::bordered())
            .draw(frame, layout[2], &mut self.message_prompt_state);

        let items: Vec<Line> = self
            .chat_messages
            .iter()
            .map(|chat_message| {
                let color = self.username_to_color.get(&chat_message.username).unwrap();
                Line::from(vec![
                    Span::styled(
                        format!(" User: {}", chat_message.username),
                        Style::default().fg(*color),
                    ),
                    Span::styled(" - ", Style::default().fg(*color)),
                    Span::styled(
                        format!("{}", chat_message.message),
                        Style::default().fg(*color),
                    ),
                ])
            })
            .collect();

        frame.render_widget(
            Paragraph::new(items.clone())
                .wrap(Wrap { trim: false })
                .block(Block::default().title("Messages").borders(Borders::ALL))
                .scroll((self.vertical_scroll, 0)),
            layout[1],
        );

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state =
            ScrollbarState::new(items.len()).position(self.vertical_scroll as usize);

        frame.render_stateful_widget(
            scrollbar,
            layout[1].inner(Margin {
                // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}
