pub fn find_max(arr: &[i32]) -> Option<(i32, usize)> {
    if arr.is_empty() {
        return None;
    }

    let mut max: i32 = arr[0];
    let mut loc: usize = 0;
    for (i, &x) in arr[1..].iter().enumerate() {
        if x > max {
            max = x;
            loc = i;
        }
    }

    Some((max, loc))
}

pub fn find_min(arr: &[i32]) -> Option<(i32, usize)> {
    if arr.is_empty() {
        return None;
    }

    let mut min: i32 = arr[0];
    let mut loc: usize = 0;
    for (i, &x) in arr[1..].iter().enumerate() {
        if x < min {
            min = x;
            loc = i;
        }
    }

    Some((min, loc))
}
