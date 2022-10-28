use super::Storage;
use crate::lib::download::Download;
use crate::models::{AddBookmark, Bookmark, QueryResponse};
use chrono::Utc;

impl Storage {
    pub async fn add_bookmark(
        &mut self,
        bookmark: AddBookmark,
        favicon: Option<Download>,
    ) -> Result<(), anyhow::Error> {
        let description: String = match &bookmark.description {
            Some(d) => d.to_string(),
            None => String::from(""),
        };

        let timestamp = Utc::now().naive_utc();
        let icon_path = match favicon {
            Some(favicon) => match self.save_favicon(favicon).await {
                Ok(icon_path) => String::from(icon_path.to_str().unwrap()),
                Err(e) => {
                    println!("ERROR-- {}", e);
                    String::from("")
                }
            },
            None => String::from(""),
        };
        println!("HERE{:?}", icon_path);

        match sqlx::query!(
            r#"
            INSERT INTO bookmarks (
                date,
                link, 
                description,
                icon_path 
                )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
            timestamp,
            bookmark.link,
            description,
            icon_path,
        )
        .fetch_one(&self.pool)
        .await
        {
            // could return Ok(rec) => rec?
            Ok(_) => {
                println!("yes");
            }
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(anyhow::anyhow!("[storage] - Error adding bookmark"));
            }
        };
        Ok(())
    }

    pub async fn get_bookmarks(
        &mut self,
        limit: usize,
        offset: usize,
        is_trashed: bool,
    ) -> QueryResponse<Bookmark> {
        let query_count = format!(
            "SELECT COUNT(*) FROM bookmarks WHERE trashed={}",
            is_trashed
        );
        println!("{}", query_count);
        // Add actual error checking here
        let res: i64 = match sqlx::query_scalar::<sqlx::Postgres, i64>(&query_count)
            .fetch_one(&self.pool)
            .await
        {
            Ok(res) => res,
            Err(_) => {
                return QueryResponse {
                    is_succesful: false,
                    records: None,
                    pages: None,
                    error: Some("Error".into()),
                }
            }
        };

        let query_statement = format!(
            "SELECT
                id,
                date,
                link,
                description,
                icon_path,
                trashed
            FROM
                bookmarks 
            WHERE
                trashed={}
                ORDER BY id DESC
                LIMIT {} OFFSET {}
            ",
            is_trashed, limit, offset
        );

        let query = sqlx::query_as::<sqlx::Postgres, Bookmark>(&query_statement);
        // catch this error
        let bookmarks = query.fetch_all(&self.pool).await.unwrap();

        QueryResponse {
            is_succesful: true,
            records: Some(bookmarks),
            pages: Some((res / limit as i64) as usize + 1),
            error: None,
        }
    }
}
