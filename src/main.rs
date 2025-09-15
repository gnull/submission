pub mod api;
pub mod db;

use actix_web::{App, HttpServer, web};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::api::*;
use crate::db::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut db = SubmDb::new("file.sqlite").await;
    db.init().await;

    let db_state = Arc::new(Mutex::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_state.clone()))
            .service(index)
            .service(create_problem)
            .service(get_problems)
            .service(get_problem)
            .service(create_submission)
            .service(get_submissions)
            .service(get_submission)
            .service(create_feedback)
            .service(get_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
