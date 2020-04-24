#[cfg(test)]
mod tests {
    use super::filter;
    use super::map;
    #[test]
    fn filter_works() {
        let unfiltered = vec![1, 2, 3, 4, 5];
        let filtered = filter(unfiltered, |n: &usize| -> bool { n % 3 == 0 });
        assert_eq!(filtered, [3]);
    }
    #[test]
    fn map_works() {
        let unmapped = vec![1, 2, 3, 4, 5, 6];
        let mapped = map(unmapped, |n: &usize| -> usize {
            println!("{}", n * n);
            return n * n;
        });
        assert_eq!(mapped, [1, 4, 9, 16, 25, 36]);
    }
}
mod filter;
mod map;

pub use filter::filter;
pub use map::map;
