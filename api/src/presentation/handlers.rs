use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};

use crate::{AppState, domain::job::Job, presentation::error::ApiError};

pub async fn get_job_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<Job>, ApiError> {
    let job = state.get_job.execute(id)?;

    Ok(Json(job))
}
