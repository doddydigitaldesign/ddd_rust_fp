/// Functional Programming Utils
/// # Examples
/// ```
/// let some_data = vec![1, 2, 3, 4, 5, 6, 7];
/// let mapped_data = ddd_rust_fp::map(some_data, |n: &usize| -> usize {
///     println!("{}", n * n);
///     return n * n;
/// });
/// assert_eq!(mapped_data, [1, 4, 9, 16, 25, 36, 49]);
///```
pub fn map<T, K>(arr: Vec<T>, f: fn(&T) -> K) -> Vec<K> {
    arr.into_iter().map(|el| f(&el)).collect()
}
