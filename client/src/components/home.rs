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

pub struct Home {
    list_items: Vec<String>,
    list_state: ListState,
}

impl Default for Home {
    fn default() -> Home {
        Self {
            list_items: vec!["Login".into(), "Register".into(), "Chat".into()],
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl Home {
    pub fn new() -> Home {
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
            println!("choose : {}", self.list_items[i]);
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
    }
}
