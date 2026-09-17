mod application;
mod domain;
mod infrastructure;

use std::sync::{Arc, atomic::AtomicU64};

use axum::{
    Router,
    extract::{Json, Path, State},
    routing::{get, post},
};

use application::create_job::CreateJobUseCase;
use domain::{
    job::{CreateJobRequest, Job},
    job_repository::JobRepository,
};
use infrastructure::in_memory_repository::InMemoryJobRepository;

struct AppState {
    create_job: Arc<CreateJobUseCase>,
    repository: Arc<dyn JobRepository>,
}

#[tokio::main]
async fn main() {
    let repository: Arc<dyn JobRepository> = Arc::new(InMemoryJobRepository::new());
    let create_job = Arc::new(CreateJobUseCase::new(
        repository.clone(),
        Arc::new(AtomicU64::new(1)),
    ));

    let state = Arc::new(AppState {
        repository: repository.clone(),
        create_job,
    });
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/jobs", post(create_job_handler))
        .route("/jobs/{id}", get(get_job))
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
    state.repository.save(job.clone());

    Json(job)
}

async fn get_job(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> Json<Option<Job>> {
    let job = state.repository.find_by_id(id);

    Json(job)
}
