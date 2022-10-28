use std::error;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
// use tui::style::{Color, Style};
use crate::database::Database;
use crate::database::SearchResult;
use colored::*;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::Text;
use tui::text::{Span, Spans};
use tui::widgets::ListState;
use tui::widgets::StatefulWidget;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Widget};
use tui_input::Input;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub input: Input,
    pub input_mode: InputMode,
    pub db: Database,
    pub search_results: Vec<SearchResult>,
    pub compiled_results: Paragraph<'static>,
    pub results_found: bool,
    pub cursor_position: i32,
}

pub enum InputMode {
    Searching,
    Selecting,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(db: Database) -> App {
        App {
            running: true,
            input: Input::default(),
            input_mode: InputMode::Searching,
            db: db,
            search_results: Vec::new(),
            compiled_results: Paragraph::new(""),
            results_found: false,
            cursor_position: -1,
        }
    }

    pub fn tick(&self) {}

    pub fn search_box<'a>(&'a mut self, rows: &Vec<Rect>) -> Paragraph {
        let width = rows[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = (self.input.cursor() as u16).max(width) - width;
        Paragraph::new(self.input.value())
            .style(match self.input_mode {
                InputMode::Searching => Style::default().fg(Color::Green),
                InputMode::Selecting => Style::default(),
            })
            .scroll((0, scroll))
            .block(Block::default().borders(Borders::ALL).title("Search"))
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        if self.cursor_position == -1 {
            self.input_mode = InputMode::Searching;
        } else {
            self.input_mode = InputMode::Selecting;
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(3), Constraint::Percentage(100)].as_ref())
            .split(frame.size());

        frame.render_widget(self.search_box(&rows), rows[0]);
        frame.render_widget(self.compiled_results.clone(), rows[1]);
    }
}
