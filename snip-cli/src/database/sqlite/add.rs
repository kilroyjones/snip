use crate::database::SqliteConnector;
use crate::database::Tag;
use crate::snippet::Snippet;
use chrono;
use rusqlite::named_params;

impl SqliteConnector {
    pub fn add(&mut self, snippets: Vec<Snippet>) {
        if snippets.len() >= 1 {
            let snippet = snippets.first().unwrap();
            let tags: Vec<Tag> = self.get_tags(snippet.tags.clone()).unwrap();
            if snippets.len() == 1 {
                let snippet_id = self.add_single(snippet).unwrap();
                let _ = self.add_tags(snippet.tags.clone());
                self.connect_tags(snippet_id, tags);
            } else {
                let group_id = self.add_group_id(snippets.len()).unwrap();
                for mut snippet in snippets {
                    snippet.group_id = group_id;
                    let snippet_id = self.add_single(&snippet).unwrap();
                    self.connect_tags(snippet_id, tags.clone());
                }
            }
        }
    }

    fn add_single(&mut self, snippet: &Snippet) -> Result<usize, ()> {
        let params = named_params! {
            ":date": format!("{}", chrono::Local::now().naive_local()),
            ":snippet": snippet.snippet,
            ":filename": snippet.filename,
            ":filetype": snippet.filetype,
            ":group_id": snippet.group_id,
            ":description": snippet.description
        };

        self.conn
            .execute(
                "INSERT INTO snippets (
                    date, 
                    snippet,
                    filename, 
                    filetype, 
                    group_id, 
                    description) 
                VALUES(
                    :date, 
                    :snippet, 
                    :filename, 
                    :filetype, 
                    :group_id, 
                    :description)",
                params,
            )
            .unwrap();
        Ok(self.conn.last_insert_rowid() as usize)
    }

    fn add_group_id(&mut self, count: usize) -> Result<usize, ()> {
        self.conn
            .execute(
                "INSERT INTO snippet_groups (count) VALUES(:count)",
                named_params! {
                    ":count": count,
                },
            )
            .unwrap();
        Ok(self.conn.last_insert_rowid() as usize)
    }

    fn connect_tags(&mut self, snippet_id: usize, tags: Vec<Tag>) {
        for tag in tags.iter() {
            self.conn
                .execute(
                    "INSERT INTO snippet_to_tags (snippet_id, tag_id) VALUES(:snippet_id, :tag_id)",
                    named_params! {
                        ":snippet_id": snippet_id.clone(),
                        ":tag_id": tag.id
                    },
                )
                .unwrap();
        }
    }

    pub fn add_tags(&mut self, tags: Vec<String>) -> Result<(), rusqlite::Error> {
        for tag in tags {
            self.conn
                .execute(
                    "INSERT OR IGNORE INTO tags (tag) VALUES(:tag)",
                    named_params! {
                        ":tag": tag.clone(),
                    },
                )
                .unwrap();
        }
        Ok(())
    }

    pub fn get_tags(&mut self, tags: Vec<String>) -> Result<Vec<Tag>, rusqlite::Error> {
        let mut existing_tags: Vec<Tag> = Vec::new();
        let mut select_statement = self
            .conn
            .prepare_cached("SELECT id, tag FROM tags WHERE tag=:tag;")
            .unwrap();

        for tag in tags {
            let mut rows = select_statement
                .query(named_params! {":tag": tag.clone()})
                .unwrap();
            while let Some(row) = rows.next()? {
                existing_tags.push(Tag {
                    id: row.get(0)?,
                    tag: row.get(1)?,
                });
            }
        }
        Ok(existing_tags)
    }
}
