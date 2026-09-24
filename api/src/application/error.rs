#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("job not found")]
    JobNotFound,
    #[error("invalid request: {0}")]
    InvalidRequest(String),
}
