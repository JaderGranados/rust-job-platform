mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::{Arc, atomic::AtomicU64};

use axum::{
    Router,
    extract::{Json, State},
    routing::{get, post},
};

use application::create_job::CreateJobUseCase;
use domain::{
    job::{CreateJobRequest, Job},
    job_repository::JobRepository,
};
use infrastructure::in_memory_repository::InMemoryJobRepository;

use crate::application::get_job::GetJobUseCase;
use crate::presentation::handlers::get_job_handler;

struct AppState {
    create_job: Arc<CreateJobUseCase>,
    get_job: Arc<GetJobUseCase>,
}

#[tokio::main]
async fn main() {
    let repository: Arc<dyn JobRepository> = Arc::new(InMemoryJobRepository::new());
    let create_job = Arc::new(CreateJobUseCase::new(
        repository.clone(),
        Arc::new(AtomicU64::new(1)),
    ));
    let get_job = Arc::new(GetJobUseCase::new(repository));

    let state = Arc::new(AppState {
        get_job,
        create_job,
    });
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/jobs", post(create_job_handler))
        .route("/jobs/{id}", get(get_job_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("API listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello from Rust!"
}

async fn health() -> &'static str {
    "Ok"
}

async fn create_job_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateJobRequest>,
) -> Json<Job> {
    let job: Job = state.create_job.execute(CreateJobRequest {
        job_type: request.job_type,
        payload: request.payload,
    });

    Json(job)
}
