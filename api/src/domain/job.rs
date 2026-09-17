use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub payload: u64,
}

#[derive(Serialize, Clone)]
pub struct Job {
    pub id: u64,
    pub job_type: String,
    pub payload: u64,
    pub status: String,
}
