use crate::task::{Task, TaskError, TaskStatus};

pub struct Action<C> {
    name: String,
    inner: Box<dyn FnMut(&mut C) -> Result<TaskStatus, TaskError> + Send>,
}

impl <C> Action<C> {
    pub fn new<S, F>(name: S, f: F) -> Self
    where 
        S: Into<String>,
        F: FnMut(&mut C) -> Result<TaskStatus, TaskError> + Send + 'static,
    {
        Self { name: name.into(), inner: Box::new(f) }
    }
}

impl <C> Task<C> for Action<C> {
    fn name(&self) -> &str {
        &self.name
    }

    fn tick(&mut self, ctx: &mut C) -> Result<TaskStatus, TaskError> {
        (self.inner)(ctx)
    }
}