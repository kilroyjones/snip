mod app;
mod document;
mod events;
mod handler;
mod interface;
mod results;

use crate::database::Database;
use crate::search_tui::app::{App, AppResult};
use crate::search_tui::events::{Event, EventHandler};
use crate::search_tui::handler::handle_key_events;
use crate::search_tui::interface::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub fn start_search(db: Database) -> AppResult<()> {
    let mut app = App::new(db);
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    tui.exit()?;
    Ok(())
}
