use crate::broadcast::Broadcaster;
use crate::lib::file::*;
use crate::lib::image::save_base64::save_base64_image;
use crate::lib::image::save_common::save_common_image;
use crate::lib::url::content_type::*;
use crate::lib::url::*;
use crate::models::{Image, SnipMessage};
use actix_web::{post, web, Responder};
use anyhow::anyhow;
use chrono::Utc;
use sqlx::postgres::PgPool;

pub fn error_message(msg: &str) -> actix_web::web::Json<SnipMessage> {
    web::Json(SnipMessage { msg: msg.into() })
}

#[post("/add-image")]
pub async fn add_image(
    image: web::Json<Image>,
    db_pool: web::Data<PgPool>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let base: String = String::from("./files/images");
    let url = image.url.clone();

    // Is reregular image or base64
    let url_type = match get_url_type(&url).await {
        Ok(url_type) => url_type,
        Err(_) => return error_message("Error"),
    };
    println!("{:?}", url_type);

    // Get http request
    let response = match reqwest::get(url.clone()).await {
        Ok(response) => Some(response),
        Err(_) => None,
    };

    // Get contennt_type
    let content_type = match get_content_type(&url, &url_type, &response).await {
        Ok(content_type) => content_type,
        Err(_) => return error_message("Error"),
    };

    // Get filename
    let filename = match get_filename(&url, &url_type, &content_type).await {
        Ok(content_type) => content_type,
        Err(_) => return error_message("Error"),
    };

    // Modify filename if another file already exists
    let filename = check_if_file_exists(&base, filename, &content_type).await;

    // Get description
    let description: String = match &image.description {
        Some(d) => d.to_string(),
        None => String::from(""),
    };

    // Save file
    match url_type {
        UrlType::COMMON => {
            match save_common_image(&base, &filename, &content_type, response).await {
                Ok(_) => (),
                Err(_) => return error_message("Error"),
            }
        }
        UrlType::BASE64 => match save_base64_image(&base, &filename, &url).await {
            Ok(_) => (),
            Err(_) => return error_message("Error"),
        },
        UrlType::UNKNOWN => return error_message("error"),
    };

    match add_to_db(
        db_pool,
        image.url.clone(),
        image.source.clone(),
        description,
        base,
        filename,
        broadcaster,
    )
    .await
    {
        Ok(_) => return web::Json(SnipMessage { msg: "Ok".into() }),
        Err(_) => return error_message("Error writing to the database"),
    }
}

async fn add_to_db(
    db_pool: web::Data<PgPool>,
    content: String,
    source: String,
    description: String,
    base: String,
    filename: String,
    broadcaster: web::Data<Broadcaster>,
) -> Result<(), anyhow::Error> {
    // Add to DB
    println!("ADD TO DB:");
    let timestamp = Utc::now().naive_utc();
    println!("{:?}, {:?}", base, filename);
    match sqlx::query!(
        r#"
            INSERT INTO snippets (
                date, 
                content, 
                source, 
                description, 
                snippet_type, 
                mimetype, 
                image_path, 
                thumbnail_path)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
        "#,
        timestamp,
        content,
        source,
        description,
        "image",
        "image/png",
        format!("{}/original/{}", base, filename),
        format!("{}/thumbnail/{}", base, filename),
    )
    .fetch_one(db_pool.get_ref())
    .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            return Err(anyhow!(e));
        }
    }
    broadcaster.broadcast("update".into(), "image".into()).await;
    Ok(())
}
