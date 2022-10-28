pub mod bookmarks;
pub mod images;
pub mod quotes;

use anyhow::anyhow;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Storage {
    pub base_path: PathBuf,
    image_original_path: PathBuf,
    image_thumbnail_path: PathBuf,
    image_favicon_path: PathBuf,
    pub pool: PgPool,
}

impl Storage {
    pub async fn new(path: String, db_connection_str: String) -> Result<Storage, anyhow::Error> {
        let base_path = PathBuf::from(path);

        let pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_connection_str)
            .await
        {
            Ok(pool) => pool,
            _ => return Err(anyhow!("[file_storage] - Couldn't connect to database")),
        };

        let image_original_path = PathBuf::from(base_path.join("images/original"));
        let image_thumbnail_path = PathBuf::from(base_path.join("images/thumbnail"));
        let image_favicon = PathBuf::from(base_path.join("images/favicon"));

        match base_path.exists() {
            true => Ok(Storage {
                base_path: base_path,
                image_original_path: image_original_path,
                image_thumbnail_path: image_thumbnail_path,
                image_favicon_path: image_favicon,
                pool: pool,
            }),
            false => Err(anyhow!("[file_storage] - Base file path cannot be found")),
        }
    }
}
