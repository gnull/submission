use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, Result, get, post, web};
use futures_util::StreamExt;

use std::sync::Arc;
use tokio::sync::{Mutex};

use crate::db::*;
use crate::error::SubmError;

use log::error;

pub type DbState = Arc<Mutex<SubmDb>>;

#[get("/api/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Submission System API",
        "version": "1.0"
    }))
}

// Problem endpoints
#[post("/api/problems")]
async fn create_problem(
    db: web::Data<DbState>,
    problem: web::Json<CreateProblem>,
) -> Result<impl Responder> {
    let db = db.lock().await;
    match db.create_problem(problem.into_inner()).await {
        Ok(id) => Ok(HttpResponse::Created().json(serde_json::json!({"id": id}))),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

#[get("/api/problems")]
async fn get_problems(db: web::Data<DbState>) -> Result<impl Responder> {
    let db = db.lock().await;
    match db.get_problems().await {
        Ok(problems) => Ok(HttpResponse::Ok().json(problems)),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

#[get("/api/problems/{id}")]
async fn get_problem(db: web::Data<DbState>, path: web::Path<i64>) -> Result<impl Responder> {
    let problem_id = path.into_inner();
    let db = db.lock().await;
    match db.get_problem_by_id(problem_id).await {
        Ok(Some(problem)) => Ok(HttpResponse::Ok().json(problem)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Problem not found"
        }))),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

// Submission endpoints
#[post("/api/problems/{id}/submissions")]
async fn create_submission(
    db: web::Data<DbState>,
    path: web::Path<i64>,
    mut payload: Multipart,
) -> Result<impl Responder> {
    let problem_id = path.into_inner();
    let mut comment = String::new();
    let mut files = Vec::new();

    // Parse multipart form data
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = field.name().to_string();

        match field_name.as_str() {
            "comment" => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    data.extend_from_slice(&chunk?);
                }
                comment = String::from_utf8(data).unwrap_or_default();
            }
            name if name.starts_with("file") => {
                let filename = field
                    .content_disposition()
                    .get_filename()
                    .unwrap_or("unknown")
                    .to_string();

                let mut file_data = Vec::new();
                while let Some(chunk) = field.next().await {
                    file_data.extend_from_slice(&chunk?);
                }
                files.push((filename, file_data));
            }
            _ => {}
        }
    }

    let submission = CreateSubmission { comment };

    // Verify problem exists
    let db = db.lock().await;
    match db.get_problem_by_id(problem_id).await {
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Problem not found"
            })));
        }
        Err(e) => {
            error!("{}", e);
            return Err(SubmError::GenericError.into());
        }
        _ => {}
    }

    match db
        .create_submission_with_files(problem_id, submission, files)
        .await
    {
        Ok(id) => Ok(HttpResponse::Created().json(serde_json::json!({"id": id}))),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

#[get("/api/submissions")]
async fn get_submissions(db: web::Data<DbState>) -> Result<impl Responder> {
    let db = db.lock().await;
    match db.get_submissions().await {
        Ok(submissions) => Ok(HttpResponse::Ok().json(submissions)),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

#[get("/api/submissions/{id}")]
async fn get_submission(db: web::Data<DbState>, path: web::Path<i64>) -> Result<impl Responder> {
    let submission_id = path.into_inner();
    let db = db.lock().await;
    match db.get_submission_by_id(submission_id).await {
        Ok(Some(submission)) => Ok(HttpResponse::Ok().json(submission)),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Submission not found"
        }))),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

#[post("/api/submissions/{id}/feedback")]
async fn create_feedback(
    db: web::Data<DbState>,
    path: web::Path<i64>,
    feedback: web::Json<CreateFeedback>,
) -> Result<impl Responder> {
    let submission_id = path.into_inner();

    // Verify submission exists
    let db = db.lock().await;
    match db.get_submission_by_id(submission_id).await {
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "Submission not found"
            })));
        }
        Err(e) => {
            error!("{}", e);
            return Err(SubmError::GenericError.into());
        }
        _ => {}
    }

    match db
        .create_feedback(submission_id, feedback.into_inner())
        .await
    {
        Ok(feedback_id) => Ok(HttpResponse::Created().json(serde_json::json!({
            "id": feedback_id,
        }))),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::GenericError.into())
        },
    }
}

#[get("/api/files/{hash}")]
async fn get_file(db: web::Data<DbState>, path: web::Path<String>) -> Result<impl Responder> {
    let hash = path.into_inner();

    let db = db.lock().await;
    match db.get_file_content(&hash).await {
        Ok(content) => Ok(HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(content)),
        Err(e) => {
            error!("{}", e);
            Err(SubmError::FileNotFound.into())
        },
    }
}
