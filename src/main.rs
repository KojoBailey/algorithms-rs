mod min_max;

fn main() {
    let arr = [4i32, 8, 22, 94, 44, 55, 92, 83, 17, -23];

    if let Some((max_v, max_l)) = min_max::find_max(&arr) {
        println!("Max: {} at {}", max_v, max_l);
    }

    if let Some((min_v, min_l)) = min_max::find_min(&arr) {
        println!("Min: {} at {}", min_v, min_l);
    }
}
