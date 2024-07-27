use crate::components::{Footer, Home};

#[derive(PartialEq)]
pub enum AppMode {
    View,
    Write,
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
}

impl<'a> Default for App<'a> {
    fn default() -> App<'a> {
        Self {
            should_quit: false,
            view: AppView::Home,
            mode: AppMode::View,
            home: Home::new(),
            footer: Footer::new(),
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
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
