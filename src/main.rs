use chess::app::{App, AppResult};
use chess::event::{Event, EventHandler};
use chess::handler::handle_key_events;
use chess::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> AppResult<()> {
    // create an application.
    let mut app = App::new();

    // initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    
    tui.init()?;

    // start the main loop.
    while app.running {
        // render the user interface.
        tui.draw(&mut app)?;
        // handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // exit the user interface.
    tui.exit()?;
    Ok(())
}