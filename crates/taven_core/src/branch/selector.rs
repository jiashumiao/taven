use crate::task::{BoxTask, Task, TaskResult, TaskStatus};

/// Selector (Fallback): 逐个子节点执行，
/// 遇到 Success -> Success，遇到 Running -> Running，
/// 全部 Failure -> Failure
pub struct Selector<C> {
    name: String,
    children: Vec<BoxTask<C>>,
    idx: usize,
}

impl<C> Selector<C> {
    pub fn new(name: impl Into<String>, children: Vec<BoxTask<C>>) -> Self {
        Self {
            name: name.into(),
            children,
            idx: 0,
        }
    }

    pub fn reset(&mut self) {
        self.idx = 0
    }
}

impl<C> Task<C> for Selector<C> {
    fn name(&self) -> &str {
        &self.name
    }

    fn tick(&mut self, ctx: &mut C) -> TaskResult {
        while self.idx < self.children.len() {
            let status = self.children[self.idx].tick(ctx)?;
            match status {
                TaskStatus::Failure => {
                    self.idx += 1;
                    continue;
                }
                TaskStatus::Running => return Ok(TaskStatus::Running),
                TaskStatus::Success => {
                    self.idx = 0;
                    return Ok(TaskStatus::Success);
                }
            }
        }
        self.idx = 0;
        Ok(TaskStatus::Failure)
    }
}
