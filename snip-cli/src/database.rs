mod sqlite;
use crate::database::sqlite::SqliteConnector;
use crate::snippet::Snippet;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: usize,
    pub tag: String,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub filename: String,
    pub description: String,
    pub preview: String,
}

pub enum DatabaseType {
    Sqlite,
    // Postgresql,
}

pub struct Database {
    pub db_type: DatabaseType,
    // pub pg_pool: Option<PgPool>
    // pub sqlite_pool: Option<SqlitePool> // Implement later
    pub sqlite: SqliteConnector,
}

impl Database {
    pub fn new(db_type: DatabaseType, connection_str: String) -> Database {
        match db_type {
            DatabaseType::Sqlite => Database {
                db_type: db_type,
                sqlite: SqliteConnector::new(connection_str),
            },
        }
    }

    pub fn add(&mut self, snippets: Vec<Snippet>) {
        match self.db_type {
            DatabaseType::Sqlite => self.sqlite.add(snippets),
        };
    }

    pub fn search(&mut self, search_term: String) -> Result<Vec<SearchResult>, ()> {
        match self.db_type {
            DatabaseType::Sqlite => return self.sqlite.search(search_term),
        };
    }
}
