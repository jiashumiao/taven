pub mod branch;
pub mod leaf;
pub mod task;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub mod prelude {
    pub use crate::add;
}
