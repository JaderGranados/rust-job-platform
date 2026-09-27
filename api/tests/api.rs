use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt;

fn app() -> Router {
    api::build_app()
}

async fn json_body(response: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn health_check_returns_ok() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_job_returns_the_created_job() {
    let request = Request::builder()
        .method("POST")
        .uri("/jobs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "job_type": "resize", "payload": 5 }).to_string(),
        ))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["job_type"], "resize");
    assert_eq!(body["status"], "pending");
}

#[tokio::test]
async fn create_job_rejects_an_empty_job_type() {
    let request = Request::builder()
        .method("POST")
        .uri("/jobs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "job_type": "", "payload": 5 }).to_string(),
        ))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn get_job_returns_a_previously_created_job() {
    let app = app();

    let create_request = Request::builder()
        .method("POST")
        .uri("/jobs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({ "job_type": "resize", "payload": 5 }).to_string(),
        ))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let created = json_body(create_response).await;
    let id = created["id"].as_u64().unwrap();

    let get_request = Request::builder()
        .uri(format!("/jobs/{id}"))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(get_request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    assert_eq!(body["id"], id);
}

#[tokio::test]
async fn get_job_returns_not_found_for_unknown_id() {
    let request = Request::builder()
        .uri("/jobs/999")
        .body(Body::empty())
        .unwrap();

    let response = app().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = json_body(response).await;
    assert_eq!(body["error"], "job_not_found");
}

#[tokio::test]
async fn list_jobs_returns_all_created_jobs() {
    let app = app();

    let first = json_body(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/jobs")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({ "job_type": "resize", "payload": 5 }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;

    let second = json_body(
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/jobs")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({ "job_type": "compress", "payload": 20 }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap(),
    )
    .await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/jobs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = json_body(response).await;
    let ids: Vec<_> = body
        .as_array()
        .unwrap()
        .iter()
        .map(|job| job["id"].clone())
        .collect();
    assert!(ids.contains(&first["id"]));
    assert!(ids.contains(&second["id"]));
}

#[tokio::test]
async fn list_jobs_returns_empty_array_when_no_jobs_exist() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/jobs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await.as_array().unwrap().len(), 0);
}
