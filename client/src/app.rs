use crate::components::{Footer, Home};

#[derive(PartialEq)]
pub enum AppMode {
    View,
    Write,
    Error,
}

#[derive(PartialEq)]
pub enum AppView {
    Home,
    Login,
    Register,
    Chat,
}

pub struct App<'a> {
    pub should_quit: bool,
    pub view: AppView,
    pub mode: AppMode,
    pub home: Home<'a>,
    pub footer: Footer,
    pub username: String,
}

impl<'a> App<'a> {
    pub async fn new() -> App<'a> {
        Self {
            should_quit: false,
            view: AppView::Home,
            mode: AppMode::View,
            home: Home::new(),
            footer: Footer::new(),
            username: String::new(),
        }
    }

    pub fn exit(&mut self) {
        self.should_quit = true
    }

    pub fn set_error_mode(&mut self) {
        self.mode = AppMode::Error
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            AppMode::View => self.mode = AppMode::Write,
            AppMode::Write => self.mode = AppMode::View,
            // don't handle error mode
            _ => {}
        }
    }
}
