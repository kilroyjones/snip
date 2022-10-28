pub mod add;
pub mod search;

use rusqlite::Connection;
use std::path::Path;

pub struct SqliteConnector {
    conn: Connection,
}

impl SqliteConnector {
    /// TODO
    ///  - Handle this more eloquently.
    pub fn new(db_path: String) -> SqliteConnector {
        if Path::new(&db_path).exists() {
            SqliteConnector {
                conn: Connection::open(db_path).unwrap(),
            }
        } else {
            panic!("Database cannont be found");
        }
    }
}
