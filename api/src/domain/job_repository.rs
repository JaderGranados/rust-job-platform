use super::job::Job;

pub trait JobRepository: Send + Sync {
    fn save(&self, job: Job);
    fn find_by_id(&self, id: u64) -> Option<Job>;
    fn find_all(&self, expression: Option<Box<dyn Fn(Job) -> bool>>) -> Vec<Job>;
}
