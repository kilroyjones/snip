mod broadcast;
mod guesslang;
mod lib;
mod models;
mod routes;
mod telemetry;
mod tests;

use crate::broadcast::Broadcaster;
use crate::lib::storage::Storage;
use crate::routes::add::*;
use crate::routes::get::*;
use crate::routes::sse::event_stream;
use crate::routes::trash_snippet::*;
use crate::telemetry::{get_subscriber, init_subscriber};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // dotenv for logger info and database
    dotenv().ok();
    let db_connection_str;
    let subscriber = get_subscriber("snip-server".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    match env::var("DATABASE_URL") {
        Ok(connection_str) => db_connection_str = connection_str,
        Err(e) => {
            println!("Couldn't find DB {}", e);
            panic!("");
        }
    };

    let storage = Arc::new(
        Storage::new("files".into(), db_connection_str)
            .await
            .unwrap(),
    );

    let data = Broadcaster::create();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .app_data(web::Data::from(Arc::clone(&data)))
            .app_data(web::Data::from(Arc::clone(&storage)))
            .service(add_quote)
            .service(add_bookmark)
            .service(add_image)
            .service(get_images)
            .service(get_quotes)
            .service(get_bookmarks)
            .service(trash_snippet)
            .service(event_stream)
            .service(Files::new("/", ".").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
