/// Return the **first** maximum value and its index in an array.
pub fn find_max<T: PartialOrd>(arr: &[T]) -> Option<(&T, usize)> {
    compare(PartialOrd::gt, &arr)
}

/// Return the **first** minimum value and its index in an array.
pub fn find_min<T: PartialOrd>(arr: &[T]) -> Option<(&T, usize)> {
    compare(PartialOrd::lt, &arr)
}

fn compare<F, T>(mut op: F, arr: &[T]) -> Option<(&T, usize)>
where
    T: PartialOrd,
    F: FnMut(&T, &T) -> bool,
{
    let (first, rest) = arr.split_first()?;

    Some(rest.iter().enumerate().fold((first, 0),
    |(best_val, best_idx), (i, x)| {
        if op(&x, &best_val) {
            (x, i + 1)
        } else {
            (best_val, best_idx)
        }
    }))
}
