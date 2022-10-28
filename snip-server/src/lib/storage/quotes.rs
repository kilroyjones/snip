use super::Storage;
use crate::lib::download::Download;
use crate::models::{AddQuote, QueryResponse, Quote};
use chrono::Utc;

impl Storage {
    pub async fn add_quote(
        &mut self,
        quote: AddQuote,
        code_type: Option<String>,
        favicon: Option<Download>,
    ) -> Result<(), anyhow::Error> {
        let description: String = match &quote.description {
            Some(d) => d.to_string(),
            None => String::from(""),
        };

        let code_type: String = match code_type {
            Some(code_type) => code_type,
            _ => String::from(""),
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

        match sqlx::query!(
            r#"
            INSERT INTO quotes (date, quote, source, description, code_type, icon_path )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
        "#,
            timestamp,
            quote.quote,
            quote.source,
            description,
            code_type,
            icon_path,
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR: {}", e);
                return Err(anyhow::anyhow!("[storage] - Error adding bookmark"));
            }
        };
        Ok(())
    }

    #[tracing::instrument(name = "Requesting quotes")]
    pub async fn get_quotes(
        &mut self,
        limit: usize,
        offset: usize,
        is_trashed: bool,
    ) -> QueryResponse<Quote> {
        let query_count = format!("SELECT COUNT(*) FROM quotes WHERE trashed={}", is_trashed);

        // Add actual error checking here
        let res: i64 = match sqlx::query_scalar::<sqlx::Postgres, i64>(&query_count)
            .fetch_one(&self.pool)
            .await
        {
            Ok(res) => res,
            Err(_) => {
                tracing::error!("Failed to get count.");
                return QueryResponse {
                    is_succesful: false,
                    records: None,
                    pages: None,
                    error: Some("Error".into()),
                };
            }
        };

        let query_statement = format!(
            "SELECT
                id,
                date,
                quote,
                source,
                code_type,
                description,
                icon_path
            FROM
                quotes
            WHERE
                trashed={}
                ORDER BY id DESC
                LIMIT {} OFFSET {}
            ",
            is_trashed, limit, offset
        );

        let query = sqlx::query_as::<sqlx::Postgres, Quote>(&query_statement);
        // catch this error
        let quotes = match query.fetch_all(&self.pool).await {
            Ok(quotes) => quotes,
            Err(e) => {
                tracing::error!("Failed to query quotes: {:?}", e);
                return QueryResponse {
                    is_succesful: false,
                    records: None,
                    pages: None,
                    error: Some("Error".into()),
                };
            }
        };

        QueryResponse {
            is_succesful: true,
            records: Some(quotes),
            pages: Some((res / limit as i64) as usize + 1),
            error: None,
        }
    }
}
