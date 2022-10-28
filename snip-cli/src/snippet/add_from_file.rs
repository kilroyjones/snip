use crate::snippet::Snippet;
use crate::Args;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

impl Snippet {
    pub fn add_from_file(mut args: Args) -> Result<Vec<Snippet>, ()> {
        // TODO
        // [ ] - Check errors for file unwrap and give appropriate warnings
        if args.tags.len() > 0 {
            args.tags = Snippet::remove_duplicate_tags_and_lowercase(args.tags);
        }
        let filename = &args.files.first().unwrap();
        if Path::new(filename.clone()).exists() == false {
            return Err(());
        }
        let path = Path::new(filename);

        let mut file = File::open(path).unwrap();
        let mut file_content = String::new();
        let _ = file.read_to_string(&mut file_content).unwrap();

        Ok(vec![Snippet {
            id: None,
            datetime: None,
            snippet: file_content,
            tags: args.tags,
            filename: filename.to_string(),
            filetype: "".into(),
            group_id: 0,
            description: args.description,
        }])
    }
}
