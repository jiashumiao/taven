use std::pin::Pin;

use crate::task::{SyncToAsyncTask, Task, TaskResult};

pub trait AsyncTask<C>: Send {
    fn name(&self) -> &str;

    fn tick_async<'a>(
        &'a mut self,
        ctx: &'a mut C,
    ) -> Pin<Box<dyn Future<Output = TaskResult> + Send +'a>>;
}

pub type BoxAsyncTask<C> = Box<dyn AsyncTask<C>>;

pub trait IntoAsyncTask<C>: Task<C> + Sized + 'static {
    fn into_async(self) -> SyncToAsyncTask<C> {
        SyncToAsyncTask::new(Box::new(self))
    }
}

impl <T, C> IntoAsyncTask<C> for T 
where 
    T: Task<C> + 'static
{
    
}