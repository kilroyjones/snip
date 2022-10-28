use crate::snippet::Snippet;
use crate::Args;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::Read;
use std::path::Path;

impl Snippet {
    pub fn add_from_files(mut args: Args) -> Result<Vec<Snippet>, ()> {
        if args.tags.len() > 0 {
            args.tags = Snippet::remove_duplicate_tags_and_lowercase(args.tags);
        }
        let mut files_not_found: Vec<String> = Vec::new();
        for filename in args.files.iter() {
            if Path::new(&filename).exists() == false {
                files_not_found.push(filename.to_string());
            }
        }

        if files_not_found.len() > 0 {
            println!("Not all files could be found: {:?}", files_not_found);
            return Err(());
        }
        let mut snippets: Vec<Snippet> = Vec::new();
        for filename in args.files.iter() {
            if Path::new(filename).exists() == false {
                return Err(());
            }
            let path = Path::new(filename);
            let mut file = File::open(path).unwrap();
            let mut file_content = String::new();
            let _ = file.read_to_string(&mut file_content).unwrap();

            snippets.push(Snippet {
                id: None,
                datetime: None,
                snippet: file_content,
                tags: args.tags.clone(),
                filename: filename.to_string(),
                filetype: "".into(),
                group_id: 0,
                description: args.description.clone(),
            });
        }
        Ok(snippets)
    }
}
