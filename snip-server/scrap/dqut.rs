use crate::broadcast::Broadcaster;
use crate::guesslang::{guess, Code};
use crate::lib::file::check_if_file_exists;
use crate::models::{Quote, SnipMessage};
use actix_web::{post, web, Responder};
use chrono::Utc;
use sqlx::postgres::PgPool;
use std::fs::File;
use std::io::Write;
use url::{Host, Url};

pub async fn get_favicon(url: String) -> String {
    let parse_url = Url::parse(&url).unwrap();
    let host = parse_url.host().unwrap();
    let favicon_url = format!("{}://{}/{}", parse_url.scheme(), host, "favicon.ico");
    println!("{}", favicon_url);
    let response = match reqwest::get(favicon_url).await {
        Ok(response) => Some(response),
        Err(_) => return String::from(""),
    };
    println!("Here");

    let base: String = String::from("./files/images");
    let filename = format!("{}", host);
    // need to check if already exists and then just get the existing file and add it
    let filename = check_if_file_exists(&base, filename, &String::from(".ico")).await;
    let path = format!("{}/original/{}.ico", base, filename);

    let content = response.unwrap().bytes().await.unwrap();
    let mut original = File::create(&path).unwrap();
    let _ = original.write_all(&content);

    "".into()
}

#[post("/add-quote")]
pub async fn add_quote(
    quote: web::Json<Quote>,
    db_pool: web::Data<PgPool>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let description: String = match &quote.description {
        Some(d) => d.to_string(),
        None => String::from(""),
    };

    let timestamp = Utc::now().naive_utc();

    let temp = quote.quote.clone();
    let code: Code = guess(temp).await.unwrap();
    // println!("code:{} - {}", code.language, code.probability);
    let _ = get_favicon(quote.source.clone()).await;

    let mut code_type = String::from("");
    if code.probability >= 0.99 {
        code_type = code.language;
    }

    match sqlx::query!(
        r#"
            INSERT INTO snippets (
                date, 
                content, 
                code_type, 
                source, 
                description,
                snippet_type, 
                mimetype)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
        "#,
        timestamp,
        quote.quote,
        code_type,
        quote.source,
        description,
        "quote",
        "text/plain"
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        // could return Ok(rec) => rec?
        Ok(_) => (),
        Err(_) => return web::Json(SnipMessage { msg: "Ok".into() }),
    };

    broadcaster.broadcast("update".into(), "quote".into()).await;
    web::Json(SnipMessage { msg: "Ok".into() })
}
