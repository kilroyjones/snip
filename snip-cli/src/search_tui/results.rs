use crate::database::SearchResult;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

fn get_filename(filename: String) -> String {
    if filename.eq("") {
        return "[no filename]".into();
    }
    format!("{}", filename)
}

fn get_description(description: String) -> String {
    format!("{}", description)
}

// Need to include this as part of the search
fn get_tags() {}

fn get_snippet_preview(preview: String) -> Vec<String> {
    let mut count = 0;
    let mut lines: Vec<String> = Vec::new();
    for line in preview.lines() {
        lines.push(line.to_string());
        count += 1;
        if count == 3 {
            break;
        }
    }
    lines
    // let mut trimmed = String::new();
    // let mut count = 0;
    // for line in preview.lines() {
    //     trimmed.push_str(line);
    //     count += 1;
    //     if count == 3 {
    //         break;
    //     }
    // }
    // format!("{}", trimmed)
}

pub fn compile_results(results: &Vec<SearchResult>, selected_idx: i32) -> Paragraph<'static> {
    let mut compiled_results: Vec<Spans> = Vec::new();
    let mut idx = 0;
    for result in results.iter() {
        // If the IDX is not selected
        let filename = get_filename(result.filename.clone());
        let description = get_description(result.description.clone());
        let preview = get_snippet_preview(result.preview.clone());

        if idx == selected_idx {
            compiled_results.push(Spans::from(vec![Span::styled(
                filename,
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::UNDERLINED),
            )]));
        } else {
            compiled_results.push(Spans::from(vec![
                Span::styled(filename, Style::default().fg(Color::LightBlue)),
                Span::styled(
                    format!(": {}", description),
                    Style::default().fg(Color::LightYellow),
                ),
            ]));
        }

        // compiled_results.push(Spans::from(vec![Span::styled(
        //     description,
        //     Style::default().fg(Color::LightYellow),
        // )]));

        for line in preview.iter() {
            compiled_results.push(Spans::from(vec![Span::styled(
                line.clone(),
                Style::default().fg(Color::LightGreen),
            )]));
        }
        compiled_results.push(Spans::from(vec![Span::raw("")]));
        idx += 1;
    }
    Paragraph::new(compiled_results).block(Block::default().borders(Borders::ALL).title("Search"))
}
