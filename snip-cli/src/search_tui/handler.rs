use crate::search_tui::app::{App, AppResult};
use crate::search_tui::results::compile_results;
use crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm as input_backend;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // exit application on ESC
        KeyCode::Esc => {
            app.running = false;
        }
        KeyCode::Down => {
            if app.search_results.len() > 0 {
                if app.cursor_position < app.search_results.len() as i32 - 1 {
                    app.cursor_position += 1;
                }
            }
        }
        KeyCode::Up => {
            if app.cursor_position > -1 {
                app.cursor_position -= 1;
            }
        }
        _ => {
            input_backend::to_input_request(Event::Key(key_event))
                .and_then(|req| app.input.handle(req));

            app.search_results.clear();
            app.search_results = app.db.search(app.input.value().into()).unwrap();
        }
    }

            app.compiled_results = compile_results(&app.search_results, app.cursor_position);
    Ok(())
}
