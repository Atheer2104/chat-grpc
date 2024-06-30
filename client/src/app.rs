#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> App {
        Self { should_quit: false }
    }

    pub fn exit(&mut self) {
        self.should_quit = true
    }
}
