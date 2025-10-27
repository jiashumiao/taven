use crate::task::{BoxTask, Task, TaskResult, TaskStatus};

/// Sequence: 子节点依次执行，遇到 Failure -> Failure，
/// 遇到 Running -> Running（保持当前 idx），全部 Success -> Success
pub struct Sequence<C> {
    name: String,
    children: Vec<BoxTask<C>>,
    idx: usize,
}

impl<C> Sequence<C> {
    pub fn new(name: impl Into<String>, children: Vec<BoxTask<C>>) -> Self {
        Self {
            name: name.into(),
            children,
            idx: 0,
        }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }
}

impl<C> Task<C> for Sequence<C> {
    fn name(&self) -> &str {
        &self.name
    }

    fn tick(&mut self, ctx: &mut C) -> TaskResult {
        while self.idx < self.children.len() {
            let status = self.children[self.idx].tick(ctx)?;
            match status {
                TaskStatus::Success => {
                    self.idx += 1;
                    continue;
                }
                TaskStatus::Running => return Ok(TaskStatus::Running),
                TaskStatus::Failure => {
                    self.idx = 0;
                    return Ok(TaskStatus::Failure);
                }
            }
        }
        self.idx = 0;
        Ok(TaskStatus::Success)
    }
}
