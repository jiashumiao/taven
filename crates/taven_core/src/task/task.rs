use crate::task::TaskResult;

pub trait Task<C>: Send {
    /// used for debug
    fn name(&self) -> &str;

    /// execute one, return status
    fn tick(&mut self, ctx: &mut C) -> TaskResult;
}

pub type BoxTask<C> = Box<dyn Task<C>>;