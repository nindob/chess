use std::error;
use crate::board::Board;

// application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// application.
#[derive(Debug)]
pub struct App {
    // is the application running?
    pub running: bool,
    // board
    pub board: Board, 
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            board: Board::default()
        }
    }
}

impl App {
    // constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    // handles the tick event of the terminal.
    pub fn tick(&self) {}

    // set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

}