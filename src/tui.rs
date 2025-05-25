use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;

// representation of a terminal user interface.
//
// it is responsible for setting up the terminal,
// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    // interface to the terminal.
    terminal: Terminal<B>,
    // terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    // constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    // initializes the terminal interface.
    //
    // it enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // define a custom panic hook to reset the terminal properties.
        // this way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    // [`Draw`] the terminal interface by [`rendering`] the widgets.
    //
    // [`Draw`]: ratatui::Terminal::draw
    // [`rendering`]: crate::ui:render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    // resets the terminal interface.
    //
    // this function is also used for the panic hook to revert
    // the terminal properties if unexpected errors occur.
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    // exits the terminal interface.
    //
    // it disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}