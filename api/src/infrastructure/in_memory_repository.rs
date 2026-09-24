use std::sync::Mutex;

use crate::domain::{job::Job, job_repository::JobRepository};

pub struct InMemoryJobRepository {
    jobs: Mutex<Vec<Job>>,
}

impl InMemoryJobRepository {
    pub fn new() -> Self {
        Self {
            jobs: Mutex::new(Vec::new()),
        }
    }
}

impl JobRepository for InMemoryJobRepository {
    fn save(&self, job: Job) {
        // A poisoned mutex here only means an earlier request panicked mid-access;
        // the stored jobs remain valid, so recovering the guard is safe.
        let mut jobs = self
            .jobs
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        jobs.push(job);
    }

    fn find_by_id(&self, id: u64) -> Option<Job> {
        self.jobs
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .find(|job| job.id == id)
            .cloned()
    }

    fn find_all(&self, expression: Option<Box<dyn Fn(Job) -> bool>>) -> Vec<Job> {
        let predicate = match expression {
            Some(predicate) => predicate,
            None => Box::new(|_: Job| true),
        };

        self.jobs
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .iter()
            .cloned()
            .filter(|job| predicate(job.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_job(id: u64) -> Job {
        Job {
            id,
            job_type: "resize".to_string(),
            payload: 42,
            status: "pending".to_string(),
        }
    }

    #[test]
    fn find_by_id_round_trips_a_saved_job() {
        let repo = InMemoryJobRepository::new();
        repo.save(sample_job(1));

        let found = repo.find_by_id(1);

        assert_eq!(found.map(|job| job.id), Some(1));
    }

    #[test]
    fn find_by_id_returns_none_when_empty() {
        let repo = InMemoryJobRepository::new();

        assert!(repo.find_by_id(1).is_none());
    }
}
