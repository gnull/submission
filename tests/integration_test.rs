use actix_web::{App, test, web};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

// Import from our crate
use submission::api::*;
use submission::db::*;

#[actix_web::test]
async fn test_full_api_workflow() {
    // Create test database
    let mut db = SubmDb::new(":memory:", unimplemented!()).await;
    db.init().await;
    let db_state = Arc::new(Mutex::new(db));

    // Create test app
    let app = test::init_service(
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
            .service(get_file),
    )
    .await;

    // Test 1: API root
    println!("Testing API root...");
    let req = test::TestRequest::get().uri("/api/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test 2: Create a problem
    println!("Creating a problem...");
    let problem_data = json!({
        "name": "Hello World Problem",
        "desc": "Write a program that prints Hello World"
    });

    let req = test::TestRequest::post()
        .uri("/api/problems")
        .set_json(&problem_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    let problem_id = body["id"].as_i64().expect("Should have problem ID");
    println!("Created problem with ID: {}", problem_id);

    // Test 3: Get all problems
    println!("Getting all problems...");
    let req = test::TestRequest::get().uri("/api/problems").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let problems: serde_json::Value = test::read_body_json(resp).await;
    assert!(problems.is_array());
    assert_eq!(problems.as_array().unwrap().len(), 1);
    println!("Found {} problems", problems.as_array().unwrap().len());

    // Test 4: Get specific problem
    println!("Getting problem by ID...");
    let req = test::TestRequest::get()
        .uri(&format!("/api/problems/{}", problem_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let problem: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(problem["name"], "Hello World Problem");
    assert_eq!(problem["desc"], "Write a program that prints Hello World");
    println!("Retrieved problem: {}", problem["name"]);

    // Test 5: Create a submission (simplified without multipart for now)
    // For this test, we'll directly test the database layer since multipart is complex in tests
    println!("Creating a submission via database...");
    let mut db_lock = db_state.lock().await;
    let submission = CreateSubmission {
        comment: "My test submission".to_string(),
    };
    let files = vec![
        ("test.py".to_string(), b"print('Hello World')".to_vec()),
        (
            "readme.txt".to_string(),
            b"This is a test submission".to_vec(),
        ),
    ];

    let submission_id = db_lock
        .create_submission_with_files(problem_id, submission, files)
        .await
        .expect("Should create submission");
    println!("Created submission with ID: {}", submission_id);
    drop(db_lock);

    // Test 6: Get all submissions
    println!("Getting all submissions...");
    let req = test::TestRequest::get()
        .uri("/api/submissions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let submissions: serde_json::Value = test::read_body_json(resp).await;
    assert!(submissions.is_array());
    assert_eq!(submissions.as_array().unwrap().len(), 1);
    println!(
        "Found {} submissions",
        submissions.as_array().unwrap().len()
    );

    // Test 7: Get specific submission
    println!("Getting submission by ID...");
    let req = test::TestRequest::get()
        .uri(&format!("/api/submissions/{}", submission_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let submission: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(submission["comment"], "My test submission");
    assert_eq!(submission["problem"], problem_id);
    assert!(submission["files"].is_array());
    assert_eq!(submission["files"].as_array().unwrap().len(), 2);
    println!(
        "Retrieved submission with {} files",
        submission["files"].as_array().unwrap().len()
    );

    // Test 8: Create first feedback (reject)
    println!("Creating first feedback (reject)...");
    let feedback_data = json!({
        "grade": 0
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/submissions/{}/feedback", submission_id))
        .set_json(&feedback_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let feedback_response: serde_json::Value = test::read_body_json(resp).await;
    let feedback1_id = feedback_response["id"]
        .as_i64()
        .expect("Should have feedback ID");
    println!("Created feedback with ID {} and grade 0", feedback1_id);

    // Test 9: Create second feedback (accept)
    println!("Creating second feedback (accept)...");
    let feedback_data = json!({
        "grade": 1
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/submissions/{}/feedback", submission_id))
        .set_json(&feedback_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let feedback_response: serde_json::Value = test::read_body_json(resp).await;
    let feedback2_id = feedback_response["id"]
        .as_i64()
        .expect("Should have feedback ID");
    println!("Created feedback with ID {} and grade 1", feedback2_id);

    // Test 10: Get submission with feedback status
    println!("Getting submission with feedback status...");
    let req = test::TestRequest::get()
        .uri(&format!("/api/submissions/{}", submission_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let submission_with_feedback: serde_json::Value = test::read_body_json(resp).await;
    assert!(submission_with_feedback["status"].is_object());
    assert_eq!(submission_with_feedback["status"]["accepted"], true);
    assert!(submission_with_feedback["status"]["feedbacks"].is_array());
    assert_eq!(
        submission_with_feedback["status"]["feedbacks"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    println!(
        "Retrieved submission with status: accepted={}, feedbacks={}",
        submission_with_feedback["status"]["accepted"],
        submission_with_feedback["status"]["feedbacks"]
            .as_array()
            .unwrap()
            .len()
    );

    // Test 11: Test file download
    println!("Testing file download...");
    let file_hash = submission_with_feedback["files"][0]["hash"]
        .as_str()
        .expect("Should have file hash");

    let req = test::TestRequest::get()
        .uri(&format!("/api/files/{}", file_hash))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let file_content = test::read_body(resp).await;
    let content_str = String::from_utf8(file_content.to_vec()).expect("Should be valid UTF-8");
    assert_eq!(content_str, "print('Hello World')");
    println!("Downloaded file content: {}", content_str);

    // Test 12: Test invalid feedback grade
    println!("Testing invalid feedback grade...");
    let invalid_feedback_data = json!({
        "grade": 2
    });

    let req = test::TestRequest::post()
        .uri(&format!("/api/submissions/{}/feedback", submission_id))
        .set_json(&invalid_feedback_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_server_error());
    println!("Invalid feedback grade correctly rejected");

    println!("All tests passed! ✅");
}

#[actix_web::test]
async fn test_submission_rejection() {
    // Test that a submission is rejected when all feedbacks are 0
    let mut db = SubmDb::new(":memory:", unimplemented!()).await;
    db.init().await;
    let db_state = Arc::new(Mutex::new(db));

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_state.clone()))
            .service(create_problem)
            .service(create_feedback)
            .service(get_submission),
    )
    .await;

    // Create a problem first
    let problem_data = json!({
        "name": "Test Problem",
        "desc": "A test problem"
    });

    let req = test::TestRequest::post()
        .uri("/api/problems")
        .set_json(&problem_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    let problem_id = body["id"].as_i64().expect("Should have problem ID");

    // Create a submission via database
    let mut db_lock = db_state.lock().await;
    let submission = CreateSubmission {
        comment: "Test submission for rejection".to_string(),
    };
    let files = vec![("test.py".to_string(), b"print('test')".to_vec())];

    let submission_id = db_lock
        .create_submission_with_files(problem_id, submission, files)
        .await
        .expect("Should create submission");
    drop(db_lock);

    // Add multiple rejecting feedbacks (all grade 0)
    println!("Adding first rejecting feedback...");
    let feedback_data = json!({"grade": 0});
    let req = test::TestRequest::post()
        .uri(&format!("/api/submissions/{}/feedback", submission_id))
        .set_json(&feedback_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    println!("Adding second rejecting feedback...");
    let feedback_data = json!({"grade": 0});
    let req = test::TestRequest::post()
        .uri(&format!("/api/submissions/{}/feedback", submission_id))
        .set_json(&feedback_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Check that submission is rejected
    println!("Checking submission status...");
    let req = test::TestRequest::get()
        .uri(&format!("/api/submissions/{}", submission_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let submission_data: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(submission_data["status"]["accepted"], false);
    assert_eq!(
        submission_data["status"]["feedbacks"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    // Verify all feedbacks are grade 0
    for feedback in submission_data["status"]["feedbacks"].as_array().unwrap() {
        assert_eq!(feedback["grade"], 0);
    }

    println!("Submission correctly rejected with all grade 0 feedbacks! ✅");
}

#[actix_web::test]
async fn test_error_cases() {
    // Create test database
    let mut db = SubmDb::new(":memory:", unimplemented!()).await;
    db.init().await;
    let db_state = Arc::new(Mutex::new(db));

    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db_state.clone()))
            .service(get_problem)
            .service(get_submission)
            .service(create_feedback),
    )
    .await;

    // Test non-existent problem
    println!("Testing non-existent problem...");
    let req = test::TestRequest::get()
        .uri("/api/problems/999")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    // Test non-existent submission
    println!("Testing non-existent submission...");
    let req = test::TestRequest::get()
        .uri("/api/submissions/999")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    // Test feedback for non-existent submission
    println!("Testing feedback for non-existent submission...");
    let feedback_data = json!({"grade": 1});
    let req = test::TestRequest::post()
        .uri("/api/submissions/999/feedback")
        .set_json(&feedback_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);

    println!("Error case tests passed! ✅");
}
