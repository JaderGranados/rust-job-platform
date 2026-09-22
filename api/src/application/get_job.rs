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
