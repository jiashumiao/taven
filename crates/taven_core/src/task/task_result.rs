use crate::task::{TaskError, TaskStatus};

pub type TaskResult = Result<TaskStatus, TaskError>;