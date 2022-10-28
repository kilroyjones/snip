// TODO
// [ ] - Validation on input
// [ ] - Instructions on valid tags and descriptions
// use crate::helpers::all_files_exist;
use crate::Args;
use anyhow::Result;
use colored::*;
use std::io::stdin;
use std::path::Path;

enum SnippetType {
    Clipboard,
    Files,
}
#[derive(Debug, Clone)]
pub struct NotFound;

fn input(prompt: String) -> String {
    let mut answer = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut answer).unwrap();
    answer = answer.to_lowercase();
    if answer.ends_with('\n') {
        answer.pop();
        if answer.ends_with('\r') {
            answer.pop();
        }
    }
    answer
}

fn do_files_exist(files: &Vec<String>) -> bool {
    for filename in files.iter() {
        if Path::new(&filename).exists() == false {
            return false;
            // files_not_found.push(filename.to_string());
        }
    }
    true
}

pub fn interactive_loop() -> Result<Args, NotFound> {
    let snippet_type = get_snippet_type();

    match snippet_type {
        SnippetType::Clipboard => {
            let answer = input(
                "Enter tags seperated by spaces or hit enter to skip: "
                    .truecolor(123, 223, 242)
                    .to_string(),
            );
            let tags: Vec<String> = answer.split_whitespace().map(String::from).collect();
            let description = input("Enter description: ".truecolor(123, 223, 242).to_string());
            Ok(Args {
                search: false,
                clipboard: false,
                interactive: true,
                files: Vec::new(),
                tags: tags,
                description: description,
            })
        }
        SnippetType::Files => {
            let answer = input(
                "Enter file(s) seperated by spaces: "
                    .truecolor(123, 223, 242)
                    .to_string(),
            );
            let files: Vec<String> = answer.split_whitespace().map(String::from).collect();
            if do_files_exist(&files) == false {
                println!("Not all files given exist.");
                return Err(NotFound);
            }
            let answer = input(
                "Enter tags seperated by spaces or hit enter to skip: "
                    .truecolor(123, 223, 242)
                    .to_string(),
            );
            let tags: Vec<String> = answer.split_whitespace().map(String::from).collect();
            let description = input("Enter description: ".truecolor(123, 223, 242).to_string());
            Ok(Args {
                search: false,
                clipboard: false,
                interactive: true,
                files: files,
                tags: tags,
                description: description,
            })
        }
    }
}

fn get_snippet_type() -> SnippetType {
    loop {
        let answer = input(format!(
            "{}{}{}{}{}",
            "Snippet from ".truecolor(123, 223, 242),
            "[c]".truecolor(244, 232, 193),
            "lipboard or ".truecolor(123, 223, 242),
            "[f]".truecolor(244, 232, 193),
            "ile?".truecolor(123, 223, 242),
        ));
        if answer.eq("c") {
            return SnippetType::Clipboard;
        } else if answer.eq("f") {
            return SnippetType::Files;
        } else {
            println!("{} is not a valid choice. ", answer);
        }
    }
}
