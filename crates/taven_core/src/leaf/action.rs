use crate::task::{Task, TaskResult};

pub struct Action<C> {
    name: String,
    inner: Box<dyn FnMut(&mut C) -> TaskResult + Send>,
}

impl<C> Action<C> {
    pub fn new<S, F>(name: S, f: F) -> Self
    where
        S: Into<String>,
        F: FnMut(&mut C) -> TaskResult + Send + 'static,
    {
        Self {
            name: name.into(),
            inner: Box::new(f),
        }
    }
}

impl<C> Task<C> for Action<C> {
    fn name(&self) -> &str {
        &self.name
    }

    fn tick(&mut self, ctx: &mut C) -> TaskResult {
        (self.inner)(ctx)
    }
}
