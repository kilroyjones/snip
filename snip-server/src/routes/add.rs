use crate::broadcast::Broadcaster;
// use crate::lib::code::get_code_type;
use crate::lib::download::Download;
use crate::lib::favicon::get_favicon;
use crate::lib::storage::Storage;
use crate::models::{AddBookmark, AddImage, AddQuote, SnipMessage};
use actix_web::{post, web, Responder};

#[post("/add-quote")]
pub async fn add_quote(
    quote: web::Json<AddQuote>,
    storage: web::Data<Storage>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let favicon: Option<Download> = get_favicon(quote.source.clone()).await;
    // let code_type = match get_code_type(quote.quote.clone()).await {
    //     Some(code_type) => Some(code_type),
    //     _ => None,
    // };
    let code_type = Some(String::from(""));

    let quote: AddQuote = quote.into_inner();
    let mut storage = storage.as_ref().clone();
    match storage.add_quote(quote, code_type, favicon).await {
        Ok(()) => {
            broadcaster.broadcast("update".into(), "quote".into()).await;
            return web::Json(SnipMessage { msg: "Ok".into() });
        }
        Err(_) => {
            println!("Error");
            return web::Json(SnipMessage {
                msg: "Error".into(),
            });
        }
    }
}

#[post("/add-image")]
pub async fn add_image(
    image: web::Json<AddImage>,
    storage: web::Data<Storage>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let image_file: Download = match Download::new(image.url.clone()).await {
        Ok(image_file) => image_file,
        Err(e) => {
            println!("error{}", e);
            return web::Json(SnipMessage {
                msg: "Error".into(),
            });
        }
    };

    let image = image.into_inner();
    let mut storage = storage.as_ref().clone();
    match storage.add_image(image, image_file).await {
        Ok(()) => broadcaster.broadcast("update".into(), "image".into()).await,
        Err(_) => {
            return web::Json(SnipMessage {
                msg: "Error".into(),
            })
        }
    }

    return web::Json(SnipMessage { msg: "Ok".into() });
}

#[post("/add-bookmark")]
pub async fn add_bookmark(
    bookmark: web::Json<AddBookmark>,
    storage: web::Data<Storage>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    let favicon: Option<Download> = get_favicon(bookmark.link.clone()).await;
    let bookmark: AddBookmark = bookmark.into_inner();
    let mut storage = storage.as_ref().clone();
    println!("shit");
    match storage.add_bookmark(bookmark, favicon).await {
        Ok(()) => {
            broadcaster
                .broadcast("update".into(), "bookmark".into())
                .await;
            return web::Json(SnipMessage { msg: "Ok".into() });
        }
        Err(_) => {
            println!("Error");
            return web::Json(SnipMessage {
                msg: "Error".into(),
            });
        }
    }
}
