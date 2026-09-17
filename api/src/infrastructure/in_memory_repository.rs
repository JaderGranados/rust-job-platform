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
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push(job);
    }

    fn find_by_id(&self, id: u64) -> Option<Job> {
        self.jobs
            .lock()
            .unwrap()
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
            .unwrap()
            .iter()
            .cloned()
            .filter(|job| predicate(job.clone()))
            .collect()
    }
}
