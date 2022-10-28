// TODO
// [ ] - Look at reimplemention optional param group_id which is currently configured such that all non-grouped ids are given the id of 0.

mod database;
mod interactive;
mod search_tui;
mod snippet;

use crate::database::{Database, DatabaseType};
use crate::snippet::Snippet;
use clap::Parser;
use dotenv::dotenv;
use std::env;
use std::io;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, action)]
    pub search: bool,

    #[clap(short, long, action)]
    pub clipboard: bool,

    #[clap(short, long, action)]
    pub interactive: bool,

    #[clap(value_parser)]
    pub files: Vec<String>,

    #[clap(short, long, multiple = true, value_parser, default_value = "")]
    pub tags: Vec<String>,

    #[clap(short, long, multiple = false, value_parser, default_value = "")]
    pub description: String,
}

fn add_snippet(mut db: Database, mut args: Args) -> Result<(), ()> {
    // TODO
    // [ ] - Complete error handling for interactive loop
    if args.interactive {
        args = match interactive::interactive_loop() {
            Ok(args) => args,
            Err(_) => return Ok(()),
        };
    }
    let snippets = Snippet::new(args).unwrap();
    db.add(snippets);
    Ok(())
}

fn main() -> anyhow::Result<(), io::Error> {
    dotenv().ok();
    let args = Args::parse();
    let db_name: String;
    let db_backend: DatabaseType;

    match env::var("DATABASE") {
        Ok(db) => {
            db_name = db;
        }
        Err(e) => {
            println!("Couldn't find DB {}", e);
            panic!("");
        }
    };

    match env::var("DATABASE_BACKEND") {
        Ok(backend) => {
            if backend.eq("sqlite") {
                db_backend = DatabaseType::Sqlite;
            } else {
                db_backend = DatabaseType::Sqlite;
            }
        }
        Err(e) => {
            println!("Couldn't determine database_backend: {}", e);
            panic!("");
        }
    };

    let db = Database::new(db_backend, db_name);
    if args.search {
        let _ = search_tui::start_search(db);
    } else {
        let _ = add_snippet(db, args);
    }
    Ok(())
}
