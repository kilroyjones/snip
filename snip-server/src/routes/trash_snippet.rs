use crate::broadcast::Broadcaster;
use crate::models::{RemoveSnippet, SnipMessage};
use actix_web::{post, web, Responder};
use sqlx::postgres::PgPool;

#[post("/trash-snippet")]
pub async fn trash_snippet(
    removed_snippet: web::Json<RemoveSnippet>,
    db_pool: web::Data<PgPool>,
    broadcaster: web::Data<Broadcaster>,
) -> impl Responder {
    // match sqlx::query!(
    //     r#"
    //         UPDATE snippets
    //         SET trashed = true
    //         WHERE id = $1
    //     "#,
    //     removed_snippet.id
    // )
    // .execute(db_pool.get_ref())
    // .await
    // {
    //     // could return Ok(rec) => rec?
    //     Ok(_) => (),
    //     Err(_) => {
    //         return web::Json(SnipMessage {
    //             msg: "Error".into(),
    //         })
    //     }
    // };

    // broadcaster.broadcast("update".into(), "quote".into()).await;
    web::Json(SnipMessage { msg: "Ok".into() })
}

// #[post("/trash-snippet")]
// pub async fn trash_sn(
//     removed_snippet: web::Json<RemoveSnippet>,
//     db_pool: web::Data<PgPool>,
// ) -> impl Responder {
//     match sqlx::query!(
//         r#"
//             UPDATE snippets
//             SET trashed = true
//             WHERE id = $1
//         "#,
//         removed_snippet.id
//     )
//     .execute(db_pool.get_ref())
//     .await
//     {
//         // could return Ok(rec) => rec?
//         Ok(_) => (),
//         Err(_) => {
//             return web::Json(SnipMessage {
//                 msg: "Error".into(),
//             })
//         }
//     };
//     web::Json(SnipMessage { msg: "Ok".into() })
// }
// // #[post("/")]
