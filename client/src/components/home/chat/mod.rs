use std::hash;

use crossterm::{event::KeyEvent, style::style};
use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Wrap,
    },
    Frame,
};
use tui_prompts::{FocusState, Prompt, State, TextPrompt, TextState};

#[derive(Clone)]
pub struct Chat<'a> {
    show_chat: bool,
    message_prompt_state: TextState<'a>,
    pub vertical_scroll: u16,
}

impl<'a> Default for Chat<'a> {
    fn default() -> Chat<'a> {
        Self {
            show_chat: false,
            message_prompt_state: TextState::default().with_focus(FocusState::Focused),
            vertical_scroll: 0,
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

    pub fn handle_submit(&self) {
        let message = self.message_prompt_state.value();

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

        let dummy_msg_1 = vec![
            Span::styled(" User: Alice", Style::default()),
            Span::styled(" - ", Style::default()),
            Span::styled("Hello World!", Style::default()),
        ];

        let dummy_msg_2 = vec![
            Span::styled(" User: Bob", Style::default()),
            Span::styled(" - ", Style::default()),
            Span::styled("Hello!", Style::default()),
        ];

        let line1 = Line::from(dummy_msg_1);
        let line2 = Line::from(dummy_msg_2);

        let items = vec![
            line1.clone(),
            line2.clone(),
            line1.clone(),
            line2.clone(),
            line1,
        ];

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
