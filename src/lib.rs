#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod filter;
mod map;

pub use filter::filter;

pub use map::map;
