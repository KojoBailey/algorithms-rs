pub fn find_max(arr: &[i32]) -> Option<(i32, usize)> {
    if arr.is_empty() {
        return None;
    }

    let result = arr.iter().enumerate().fold((arr[0], 0), |(max_val, max_idx), (i, &x)| {
        if x > max_val {
            (x, i)
        } else {
            (max_val, max_idx)
        }
    });

    Some(result)
}

pub fn find_min(arr: &[i32]) -> Option<(i32, usize)> {
    if arr.is_empty() {
        return None;
    }

    let result = arr.iter().enumerate().fold((arr[0], 0), |(min_val, min_idx), (i, &x)| {
        if x < min_val {
            (x, i)
        } else {
            (min_val, min_idx)
        }
    });

    Some(result)
}
