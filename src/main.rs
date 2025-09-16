pub mod api;
pub mod db;
pub mod error;

use actix_files::{Files, NamedFile};
use actix_web::{App, HttpServer, Result, middleware, web};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex};

use crate::api::*;
use crate::db::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long, default_value = "file.sqlite")]
    database: String,

    #[arg(short, long, default_value = "uploads")]
    uploads: String,

    #[arg(short, long, default_value = "./static")]
    static_dir: String,
}

// Handler for SPA routing - serves index.html for any non-API route
async fn spa_handler(static_dir: web::Data<String>) -> Result<NamedFile> {
    let path: PathBuf = format!("{}/index.html", static_dir.get_ref())
        .parse()
        .unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Args { host, port, database, uploads, static_dir } = Args::parse();

    println!("Starting server on {}:{}", &host, &port);
    println!("Database: {}", &database);
    println!("Uploads directory: {}", &uploads);
    println!("Static files directory: {}", &static_dir);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = SubmDb::new(&database, uploads).await;
    state.init().await;
    let state = Arc::new(Mutex::new(state));

    let assets_path = format!("{}/assets", &static_dir);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(static_dir.clone()))
            // API routes - these take priority over static files
            .service(index)
            .service(create_problem)
            .service(get_problems)
            .service(get_problem)
            .service(create_submission)
            .service(get_submissions)
            .service(get_submission)
            .service(create_feedback)
            .service(get_file)
            // Serve static files from the built frontend
            .service(Files::new("/assets", &assets_path))
            .service(Files::new("/", &static_dir).index_file("index.html"))
            // Catch-all handler for SPA routing (must be last)
            .default_service(web::get().to(spa_handler))
            .wrap(middleware::Logger::default())
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
