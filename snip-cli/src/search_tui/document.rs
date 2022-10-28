use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn get_formatted_document() -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let path = Path::new("b.py");
    let mut file = File::open(path).unwrap();
    let mut file_content = String::new();
    let _ = file.read_to_string(&mut file_content).unwrap();

    let syntax = ps.find_syntax_by_extension("py").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let mut out: String = String::new();
    for line in LinesWithEndings::from(file_content.as_str()) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        out = format!("{}{}", out, escaped);
        // println!("{}", escaped);
    }
    // out
    file_content
}
