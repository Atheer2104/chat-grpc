#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> App {
        Self { should_quit: false }
    }
}

impl App {
    pub fn new() -> App {
        Self::default()
    }

    pub fn exit(&mut self) {
        self.should_quit = true
    }
}
