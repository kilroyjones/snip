pub mod add_from_clipboard;
pub mod add_from_file;
pub mod add_from_files;
use crate::Args;

#[derive(Debug)]
pub struct Snippet {
    pub id: Option<usize>,
    pub datetime: Option<String>,
    pub snippet: String,
    pub tags: Vec<String>,
    pub filename: String,
    pub filetype: String,
    pub group_id: usize,
    pub description: String,
}

impl Snippet {
    pub fn new(args: Args) -> Result<Vec<Snippet>, ()> {
        if args.clipboard {
            let snippet = Snippet::add_from_clipboard(args).unwrap();
            return Ok(snippet);
        } else if args.files.len() == 1 {
            let snippet = Snippet::add_from_file(args).unwrap();
            return Ok(snippet);
        } else {
            let snippet = Snippet::add_from_files(args).unwrap();
            return Ok(snippet);
        }
    }

    pub fn remove_duplicate_tags_and_lowercase(mut tags: Vec<String>) -> Vec<String> {
        tags = tags.iter().map(|s| s.to_lowercase()).collect();
        tags.sort();
        tags.dedup();
        tags
    }
}
