pub mod favicon;
pub mod original;
pub mod thumbnail;
pub mod utils;

use super::Storage;
use crate::lib::download::Download;
use crate::models::{AddImage, Image, QueryResponse};
use anyhow::anyhow;
use chrono::Utc;
use std::path::PathBuf;

impl Storage {
    pub async fn add_image(
        &mut self,
        image: AddImage,
        image_file: Download,
    ) -> Result<(), anyhow::Error> {
        println!("{:?}\n", image);
        let description: String = match &image.description {
            Some(d) => d.to_string(),
            None => String::from(""),
        };

        let timestamp = Utc::now().naive_utc();
        let original_path: PathBuf = match self.save_image(&image_file).await {
            Ok(original_path) => original_path,
            Err(e) => {
                println!("TError saving at in iamge #remove this error later {:?}", e);
                return Err(anyhow!("[storage/images] - Error getting original image"));
            }
        };

        println!("{:?}", &original_path);
        println!("SUCCESS");
        let thumbnail_path: PathBuf = match self.save_thumbnail(&image_file).await {
            Ok(thumbnail_path) => thumbnail_path,
            Err(e) => {
                println!("SError saving at in iamge #remove this error later {:?}", e);
                return Err(anyhow!("[storage/images] - Error getting thumbnail image"));
            }
        };
        println!("SUCCESS {:?}", thumbnail_path);

        let original_path: String = original_path.display().to_string();
        let thumbnail_path: String = thumbnail_path.display().to_string();

        match sqlx::query!(
            r#"
            INSERT INTO images (
                date,
                source,
                description,
                original_path,
                thumbnail_path
                )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#,
            timestamp,
            image.source,
            description,
            original_path,
            thumbnail_path
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

    pub async fn get_images(
        &mut self,
        limit: usize,
        offset: usize,
        is_trashed: bool,
    ) -> QueryResponse<Image> {
        let query_count = format!("SELECT COUNT(*) FROM images WHERE trashed={}", is_trashed);
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
                source,
                description,
                original_path,
                thumbnail_path,
                trashed
            FROM
                images 
            WHERE
                trashed={}
                ORDER BY id DESC
                LIMIT {} OFFSET {}
            ",
            is_trashed, limit, offset
        );

        let query = sqlx::query_as::<sqlx::Postgres, Image>(&query_statement);
        // catch this error
        let images = query.fetch_all(&self.pool).await.unwrap();

        QueryResponse {
            is_succesful: true,
            records: Some(images),
            pages: Some((res / limit as i64) as usize + 1),
            error: None,
        }
    }
}
