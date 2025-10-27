#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Success,
    Failure,
    Running,
}
