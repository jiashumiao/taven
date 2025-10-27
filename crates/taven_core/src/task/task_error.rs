#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("execution error: {0}")]
    Execution(String)
}
