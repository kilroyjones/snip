use crate::broadcast::Broadcaster;
use crate::models::{Bookmark, SnipMessage};
use actix_web::{post, web, Responder};
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

#[post("/add-bookmark")]
pub async fn add_bookmark(
    bookmark: web::Json<Bookmark>,
    db_pool: web::Data<PgPool>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let description: String = match &bookmark.description {
        Some(d) => d.to_string(),
        None => String::from(""),
    };
    let timestamp = NaiveDateTime::from_timestamp(61, 0);

    match sqlx::query!(
        r#"
            INSERT INTO snippets (date, content, source, description, snippet_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#,
        timestamp,
        bookmark.link,
        bookmark.source,
        description,
        "bookmark"
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        // could return Ok(rec) => rec?
        Ok(_) => (),
        Err(_) => return web::Json(SnipMessage { msg: "Ok".into() }),
    };

    broadcaster
        .broadcast("update".into(), "bookmark".into())
        .await;
    web::Json(SnipMessage { msg: "Ok".into() })
}
