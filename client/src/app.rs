use crate::components::{Footer, Home};

#[derive(PartialEq)]
pub enum AppMode {
    View,
    Write,
}

#[derive(PartialEq)]
pub enum AppView {
    Home,
    Chat,
}

pub struct App {
    pub should_quit: bool,
    pub view: AppView,
    pub mode: AppMode,
    pub home: Home,
    pub footer: Footer,
}

impl Default for App {
    fn default() -> App {
        Self {
            should_quit: false,
            view: AppView::Home,
            mode: AppMode::View,
            home: Home::new(),
            footer: Footer::new(),
        }
    }
}

impl App {
    pub fn new() -> App {
        Self::default()
    }

    pub fn exit(&mut self) {
        self.should_quit = true
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            AppMode::View => self.mode = AppMode::Write,
            AppMode::Write => self.mode = AppMode::View,
        }
    }
}
