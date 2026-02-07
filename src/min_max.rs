pub fn find_max<T: PartialOrd>(arr: &[T]) -> Option<(&T, usize)> {
    compare(PartialOrd::gt, &arr)
}

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
    |(min_val, min_idx), (i, x)| {
        if op(&x, &min_val) {
            (x, i + 1)
        } else {
            (min_val, min_idx)
        }
    }))
}
