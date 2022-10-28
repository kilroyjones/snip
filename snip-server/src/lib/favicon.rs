use crate::broadcast::Broadcaster;
use crate::lib::download::Download;
use crate::lib::storage::Storage;
use crate::models::{Bookmark, SnipMessage};
use actix_web::{post, web, Responder};
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use url::Url;

#[tracing::instrument(name = "Saving icon")]
pub async fn get_favicon(url: String) -> Option<Download> {
    println!("{}", url);
    let parse_url = Url::parse(&url).unwrap();
    let host = parse_url.host().unwrap();
    println!("{}", host);
    let favicon_url = format!("{}://{}/{}", parse_url.scheme(), host, "favicon.ico");
    match Download::new(favicon_url).await {
        Ok(favicon) => Some(favicon),
        Err(e) => {
            tracing::error!("Unable to find favicon {}", e);
            return None;
        }
    }
}
