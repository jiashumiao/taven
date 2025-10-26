pub use taven_internal::*;

#[cfg(test)]
mod tests {
    use taven_internal::prelude::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
