use crate::database::SearchResult;
use crate::database::SqliteConnector;
use rusqlite::named_params;

impl SqliteConnector {
    pub fn get_from(&mut self, search_term: String) -> Result<Vec<SearchResult>, rusqlite::Error> {
        let mut results: Vec<SearchResult> = Vec::new();
        if search_term.len() > 0 {
            let mut select_statement = self.conn
                .prepare(format!("SELECT * FROM snippets_fts WHERE snippet MATCH :search_term OR description MATCH :search_term").as_str())?;

            let mut rows = match select_statement
                .query(named_params! {":search_term": format!("\"{}\"", search_term)})
            {
                Ok(rows) => rows,
                Err(e) => {
                    return Err(e);
                }
            };

            while let Some(row) = rows.next().unwrap() {
                let filename: String = row.get(0).unwrap();
                let snippet: String = row.get(1).unwrap();
                let description: String = row.get(2).unwrap();
                results.push(SearchResult {
                    filename: filename,
                    preview: snippet,
                    description: description,
                });
            }
        }
        Ok(results)
    }

    pub fn search(&mut self, search_term: String) -> Result<Vec<SearchResult>, ()> {
        let results = match self.get_from(search_term) {
            Ok(results) => results,
            Err(_) => return Err(()),
        };
        Ok(results)
    }
}
