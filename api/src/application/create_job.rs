use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::domain::{
    job::{CreateJobRequest, Job},
    job_repository::JobRepository,
};

pub struct CreateJobUseCase {
    repository: Arc<dyn JobRepository>,
    next_job_id: Arc<AtomicU64>,
}

impl CreateJobUseCase {
    pub fn new(repository: Arc<dyn JobRepository>, next_job_id: Arc<AtomicU64>) -> Self {
        Self {
            repository,
            next_job_id,
        }
    }

    pub fn execute(&self, request: CreateJobRequest) -> Job {
        let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);

        let job = Job {
            id,
            job_type: request.job_type,
            payload: request.payload,
            status: "pending".to_string(),
        };

        self.repository.save(job.clone());

        job
    }
}
