use crate::broadcast::Broadcaster;
use actix_web::{get, web, Responder};

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}
