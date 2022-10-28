use crate::database::Database;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusqlite::Connection;
use rusqlite::{named_params, params};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

struct SearchResults {
    results: Vec<String>,
}

impl SearchResults {
    fn new() -> SearchResults {
        SearchResults {
            results: Vec::new(),
        }
    }
}

pub fn start_search(db: &Database) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);

    let app = App::default(db);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = search_run_loop(&mut terminal, app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

fn search_run_loop<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| search_layout(frame, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.input.reset();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        input_backend::to_input_request(Event::Key(key))
                            .and_then(|req| app.input.handle(req));
                        let res = app.search_db(app.input.to_string());
                    }
                },
            }
        }
    }
}

enum InputMode {
    Normal,
    Editing,
}

fn search_layout<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let block1 = Block::default().title("Search").borders(Borders::ALL);
    let block2 = Block::default().title("Preview").borders(Borders::ALL);
    let block3 = Block::default().title("Test").borders(Borders::ALL);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let size = frame.size();
    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(size);

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(chunks[0]);
    let search_request: Input = "".into();
    let width = body_chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = (app.input.cursor() as u16).max(width) - width;
    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, scroll))
        .block(Block::default().borders(Borders::ALL).title("Search"));

    let mut res: Vec<ListItem> = Vec::new();
    for item in app.results.clone() {
        res.push(ListItem::new(item));
    }
    println!("ii{}ii", res.len());

    let items = List::new(res)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}
        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor(
                // Put cursor past the end of the input text
                body_chunks[0].x + (app.input.cursor() as u16).min(width) + 1,
                // Move one line down, from the border to the input line
                body_chunks[0].y + 1,
            )
        }
    }
    // Body & Help
    frame.render_widget(block2, chunks[1]);
    frame.render_widget(input, body_chunks[0]);
    frame.render_widget(items, body_chunks[1]);
}
