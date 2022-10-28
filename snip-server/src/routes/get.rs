use crate::lib::storage::Storage;
use crate::models::Snippets;
use actix_web::{post, web, Responder};

#[tracing::instrument(name = "Requesting quotes")]
#[post("/get-quotes")]
pub async fn get_quotes(
    snippets: web::Json<Snippets>,
    storage: web::Data<Storage>,
) -> impl Responder {
    let mut storage = storage.as_ref().clone();
    let quotes = storage
        .get_quotes(snippets.limit, snippets.offset, false)
        .await;
    web::Json(quotes)
}

#[post("/get-images")]
pub async fn get_images(
    snippets: web::Json<Snippets>,
    storage: web::Data<Storage>,
) -> impl Responder {
    let mut storage = storage.as_ref().clone();
    let images = storage
        .get_images(snippets.limit, snippets.offset, false)
        .await;
    web::Json(images)
}

#[post("/get-bookmarks")]
pub async fn get_bookmarks(
    snippets: web::Json<Snippets>,
    storage: web::Data<Storage>,
) -> impl Responder {
    let mut storage = storage.as_ref().clone();
    println!("here");
    let bookmarks = storage
        .get_bookmarks(snippets.limit, snippets.offset, false)
        .await;
    web::Json(bookmarks)
}
