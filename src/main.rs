fn find_max(arr: &[i32]) -> Option<(i32, usize)> {
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

fn find_min(arr: &[i32]) -> Option<(i32, usize)> {
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

fn main() {
    let arr = [4i32, 8, 22, 94, 44, 55, 92, 83, 17, -23];

    if let Some((max_v, max_l)) = find_max(&arr) {
        println!("Max: {} at {}", max_v, max_l);
    }

    if let Some((min_v, min_l)) = find_min(&arr) {
        println!("Min: {} at {}", min_v, min_l);
    }
}
