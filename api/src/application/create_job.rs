use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::{
    application::error::ApplicationError,
    domain::{
        job::{CreateJobRequest, Job},
        job_repository::JobRepository,
    },
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

    pub fn execute(&self, request: CreateJobRequest) -> Result<Job, ApplicationError> {
        if request.job_type.trim().is_empty() {
            return Err(ApplicationError::InvalidRequest(
                "job_type must not be empty".to_string(),
            ));
        }

        let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);

        let job = Job {
            id,
            job_type: request.job_type,
            payload: request.payload,
            status: "pending".to_string(),
        };

        self.repository.save(job.clone());

        Ok(job)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::in_memory_repository::InMemoryJobRepository;

    fn use_case() -> CreateJobUseCase {
        CreateJobUseCase::new(
            Arc::new(InMemoryJobRepository::new()),
            Arc::new(AtomicU64::new(1)),
        )
    }

    #[test]
    fn creates_a_pending_job_with_incrementing_ids() {
        let use_case = use_case();

        let first = use_case
            .execute(CreateJobRequest {
                job_type: "resize".to_string(),
                payload: 10,
            })
            .unwrap();
        let second = use_case
            .execute(CreateJobRequest {
                job_type: "resize".to_string(),
                payload: 20,
            })
            .unwrap();

        assert_eq!(first.id, 1);
        assert_eq!(second.id, 2);
        assert_eq!(first.status, "pending");
    }

    #[test]
    fn rejects_an_empty_job_type() {
        let use_case = use_case();

        let result = use_case.execute(CreateJobRequest {
            job_type: "   ".to_string(),
            payload: 10,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidRequest(_))));
    }
}
