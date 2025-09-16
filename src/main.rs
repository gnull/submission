pub mod api;
pub mod db;

use actix_files::{Files, NamedFile};
use actix_web::{App, HttpServer, Result, middleware, web};
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::api::*;
use crate::db::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port to bind to
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Path to SQLite database file
    #[arg(short, long, default_value = "file.sqlite")]
    database: String,

    /// Path to uploads directory
    #[arg(short, long, default_value = "uploads")]
    uploads: String,

    /// Path to static frontend files
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
    let args = Args::parse();

    let mut db = SubmDb::new(&args.database, &args.uploads).await;
    db.init().await;

    let db_state = Arc::new(Mutex::new(db));
    let static_dir = web::Data::new(args.static_dir.clone());

    println!("Starting server on {}:{}", args.host, args.port);
    println!("Database: {}", args.database);
    println!("Uploads directory: {}", args.uploads);
    println!("Static files directory: {}", args.static_dir);

    HttpServer::new(move || {
        let assets_path = format!("{}/assets", args.static_dir);
        App::new()
            .app_data(web::Data::new(db_state.clone()))
            .app_data(static_dir.clone())
            .wrap(middleware::Logger::default())
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
            .service(Files::new("/assets", &assets_path).show_files_listing())
            .service(Files::new("/", &args.static_dir).index_file("index.html"))
            // Catch-all handler for SPA routing (must be last)
            .default_service(web::get().to(spa_handler))
    })
    .bind((args.host.as_str(), args.port))?
    .run()
    .await
}
