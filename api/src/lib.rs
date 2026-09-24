pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use std::sync::{Arc, atomic::AtomicU64};

use axum::{
    Router,
    extract::{Json, State},
    routing::{get, post},
};

use application::{create_job::CreateJobUseCase, get_job::GetJobUseCase};
use domain::{
    job::{CreateJobRequest, Job},
    job_repository::JobRepository,
};
use infrastructure::in_memory_repository::InMemoryJobRepository;
use presentation::{error::ApiError, handlers::get_job_handler};

pub struct AppState {
    pub create_job: Arc<CreateJobUseCase>,
    pub get_job: Arc<GetJobUseCase>,
}

pub fn build_app() -> Router {
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

    Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/jobs", post(create_job_handler))
        .route("/jobs/{id}", get(get_job_handler))
        .with_state(state)
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
) -> Result<Json<Job>, ApiError> {
    let job: Job = state.create_job.execute(CreateJobRequest {
        job_type: request.job_type,
        payload: request.payload,
    })?;

    Ok(Json(job))
}
