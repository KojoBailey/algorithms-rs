pub fn find_max<T: PartialOrd>(arr: &[T]) -> Option<(&T, usize)> {
    let (first, rest) = arr.split_first()?;

    Some(rest.iter().enumerate().fold((first, 0),
    |(max_val, max_idx), (i, x)| {
        if x > max_val {
            (x, i + 1)
        } else {
            (max_val, max_idx)
        }
    }))
}

pub fn find_min<T: PartialOrd>(arr: &[T]) -> Option<(&T, usize)> {
    let (first, rest) = arr.split_first()?;

    Some(rest.iter().enumerate().fold((first, 0),
    |(min_val, min_idx), (i, x)| {
        if x < min_val {
            (x, i + 1)
        } else {
            (min_val, min_idx)
        }
    }))
}
