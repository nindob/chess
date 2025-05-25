use std::error;

// application result type
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// application
#[derive(Debug)]
pub struct App {
    // is the app running
    pub running: bool,
    // counter
    pub counter: u8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
        }
    }
}

impl App {
    // create a new app instance
    pub fn new() -> Self {
        Self::default()
    }

    // handle tick event of terminal
    pub fn tick(&self) {}

    // set running to false to quit the app
    pub fn quit(&mut self) {
        self.running = false;
    }
}