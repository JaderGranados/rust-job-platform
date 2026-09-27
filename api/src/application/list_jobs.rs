use std::sync::Arc;

use crate::domain::{job::Job, job_repository::JobRepository};

pub struct ListJobsUseCase {
    repository: Arc<dyn JobRepository>,
}

impl ListJobsUseCase {
    pub fn new(repository: Arc<dyn JobRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Vec<Job> {
        self.repository.find_all(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::job::Job, infrastructure::in_memory_repository::InMemoryJobRepository};

    #[test]
    fn returns_empty_vec_when_no_jobs() {
        let use_case = ListJobsUseCase::new(Arc::new(InMemoryJobRepository::new()));

        let jobs = use_case.execute();

        assert!(jobs.is_empty());
    }

    #[test]
    fn returns_all_saved_jobs() {
        let repository = Arc::new(InMemoryJobRepository::new());
        repository.save(Job {
            id: 1,
            job_type: "resize".to_string(),
            payload: 10,
            status: "pending".to_string(),
        });
        repository.save(Job {
            id: 2,
            job_type: "compress".to_string(),
            payload: 20,
            status: "pending".to_string(),
        });
        let use_case = ListJobsUseCase::new(repository);

        let jobs = use_case.execute();

        assert_eq!(jobs.len(), 2);
        assert!(jobs.iter().any(|j| j.id == 1));
        assert!(jobs.iter().any(|j| j.id == 2));
    }
}
