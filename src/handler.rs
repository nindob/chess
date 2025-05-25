use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application on `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // counter handlers
        KeyCode::Right => {
            app.board.cursor_right()
        }
        KeyCode::Left => {
            app.board.cursor_left()
        }
        KeyCode::Up => {
            app.board.cursor_up()
        }
        KeyCode::Down => {
            app.board.cursor_down()
        }
        KeyCode::Char(' ') => {
            app.board.select_cell()
        }
        KeyCode::Esc => {
            app.board.unselect_cell()
        }
        // other handlers you could add here.
        _ => {}
    }
    Ok(())
}