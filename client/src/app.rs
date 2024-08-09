use chat::chat::ChatMessage;

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
    pub access_token: String,
}

impl<'a> App<'a> {
    pub async fn new() -> App<'a> {
        Self {
            should_quit: false,
            view: AppView::Home,
            mode: AppMode::View,
            home: Home::new(),
            footer: Footer::new(),
            access_token: String::from("eyJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6InRlc3QyNkBnbWFpbCIsImV4cCI6IjE3MjMzOTk5NDYiLCJpYXQiOiIxNzIyNzk1MTQ2IiwiaXNzIjoiQ2hhdC1nUlBDIiwic3ViIjoiYXV0aCB0b2tlbiIsInVzZXJfaWQiOiIyNCIsInVzZXJuYW1lIjoidGVzdDI2In0.2API8Y6AVP4w4oHtNnjpWxgvU45PQUUnl6ak4iz0L5dRdWDWZcSI1CThUtHfxfuRfk1Fs8Gc8_ItjvSAQ2pHIQ"),
        }
    }

    pub fn exit(&mut self) {
        self.should_quit = true
    }

    pub fn set_error_mode(&mut self) {
        self.mode = AppMode::Error
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
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
