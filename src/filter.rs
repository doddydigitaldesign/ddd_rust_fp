/// Functional Programming Utils
/// # Examples
/// ```
/// let some_data = vec![1, 2, 3, 4, 5, 6, 7];
/// let filtered_data = ddd_rust_fp::filter(some_data, |n: &usize| -> bool {
///     n % 2 == 0
/// });
/// assert_eq!(filtered_data, [2, 4, 6]);
///```
pub fn filter<T>(arr: Vec<T>, f: fn(&T) -> bool) -> Vec<T> {
    arr.into_iter().filter(|el| f(&el)).collect()
}
