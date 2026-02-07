pub fn find_max(arr: &[i32]) -> Option<(i32, usize)> {
    let (first, rest) = arr.split_first()?;
    
    let result = rest.iter().enumerate().fold((*first, 0), |(max_val, max_idx), (i, &x)| {
        if x > max_val {
            (x, i + 1)
        } else {
            (max_val, max_idx)
        }
    });

    Some(result)
}

pub fn find_min(arr: &[i32]) -> Option<(i32, usize)> {
    let (first, rest) = arr.split_first()?;

    let result = rest.iter().enumerate().fold((*first, 0), |(min_val, min_idx), (i, &x)| {
        if x < min_val {
            (x, i + 1)
        } else {
            (min_val, min_idx)
        }
    });

    Some(result)
}
