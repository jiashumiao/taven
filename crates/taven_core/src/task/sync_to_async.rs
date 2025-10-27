use std::pin::Pin;

use crate::task::{AsyncTask, BoxTask, TaskResult};

/// Sync -> Async adapter
pub struct SyncToAsyncTask<C> {
    inner: BoxTask<C>,
}

impl <C> SyncToAsyncTask<C> {
    pub fn new(inner: BoxTask<C>) -> Self {
        Self { inner }
    }
}

impl <C> AsyncTask<C> for SyncToAsyncTask<C>
where 
    C: Send
{
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn tick_async<'a>(
            &'a mut self,
            ctx: &'a mut C,
        ) -> Pin<Box<dyn Future<Output = TaskResult> + Send +'a>> {
        // 把 sync tick 包成立即完成的 future
        Box::pin(async move { self.inner.tick(ctx) })
    }
}

