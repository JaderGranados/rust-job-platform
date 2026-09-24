use std::sync::Arc;

use crate::{
    application::error::ApplicationError,
    domain::{job::Job, job_repository::JobRepository},
};

pub struct GetJobUseCase {
    repository: Arc<dyn JobRepository>,
}

impl GetJobUseCase {
    pub fn new(repository: Arc<dyn JobRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: u64) -> Result<Job, ApplicationError> {
        self.repository
            .find_by_id(id)
            .ok_or(ApplicationError::JobNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::job::Job, infrastructure::in_memory_repository::InMemoryJobRepository};

    #[test]
    fn returns_the_job_when_found() {
        let repository = Arc::new(InMemoryJobRepository::new());
        repository.save(Job {
            id: 1,
            job_type: "resize".to_string(),
            payload: 10,
            status: "pending".to_string(),
        });
        let use_case = GetJobUseCase::new(repository);

        let job = use_case.execute(1).unwrap();

        assert_eq!(job.id, 1);
    }

    #[test]
    fn returns_job_not_found_when_missing() {
        let use_case = GetJobUseCase::new(Arc::new(InMemoryJobRepository::new()));

        let result = use_case.execute(1);

        assert!(matches!(result, Err(ApplicationError::JobNotFound)));
    }
}
