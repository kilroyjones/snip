use crate::snippet::Snippet;
use crate::Args;
use anyhow::Result;
use arboard::Clipboard;

impl Snippet {
    pub fn add_from_clipboard(mut args: Args) -> Result<Vec<Snippet>, ()> {
        // TODO
        // [ ] - Error checking on unwraps
        if args.files.len() > 0 {
            println!("ERROR: You cannot pass files along with the clipboard.");
            return Err(());
        }
        let mut clipboard = Clipboard::new().unwrap();
        let clipboard_content = clipboard.get_text().unwrap();
        if args.tags.len() > 0 {
            args.tags = Snippet::remove_duplicate_tags_and_lowercase(args.tags);
        }

        Ok(vec![Snippet {
            id: None,
            datetime: None,
            snippet: clipboard_content,
            tags: args.tags,
            filename: "".into(),
            filetype: "".into(),
            group_id: 0,
            description: args.description,
        }])
    }
}
