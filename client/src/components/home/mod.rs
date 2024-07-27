mod login;
mod register;

pub use login::*;
pub use register::*;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::Text;
use ratatui::widgets::ListState;
use ratatui::Frame;
use ratatui::{
    style::{Color, Style},
    widgets::{List, ListDirection},
};
use tui_big_text::{BigText, PixelSize};

const TITLE: &str = "Chat gRPC";

enum Action {
    Login,
    Register,
    Chat,
}

pub struct Home<'a> {
    list_items: Vec<String>,
    list_state: ListState,
    selected_action: Option<Action>,
    pub login: Login<'a>,
}

impl<'a> Default for Home<'a> {
    fn default() -> Home<'a> {
        Self {
            list_items: vec!["Login".into(), "Register".into(), "Chat".into()],
            list_state: ListState::default().with_selected(Some(0)),
            selected_action: None,
            login: Login::new(),
        }
    }
}

impl<'a> Home<'a> {
    pub fn new() -> Home<'a> {
        Self::default()
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select(&mut self) {
        if let Some(i) = self.list_state.selected() {
            // println!("choose : {}", self.list_items[i]);
            if i == 0 {
                self.login.toggle_login();
                self.selected_action = Some(Action::Login);
            } else if i == 1 {
                // register actions
            } else if i == 2 {
                // chat action
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 3), Constraint::Min(1)])
            .vertical_margin(5)
            .split(area);

        let title_paragraph = BigText::builder()
            .pixel_size(PixelSize::Full)
            .centered()
            .lines(vec![TITLE.into()])
            .build();

        frame.render_widget(title_paragraph, layout[0]);

        let items: Vec<Text> = self
            .list_items
            .iter()
            .map(|item| {
                Text::from(item.clone())
                    .centered()
                    .style(Style::default().bold())
            })
            .collect();

        let list = List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().reversed())
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, layout[1], &mut self.list_state);

        if self.login.show_login() {
            self.login.render(frame, area)
        }
    }
}
