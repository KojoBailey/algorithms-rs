mod min_max;

fn main() {
    let arr_i32 = [4i32, 8, 22, 94, 44, 55, 92, 83, 17, -23];

    println!("arr_i32: {:?}", arr_i32);

    if let Some((val, idx)) = min_max::find_min(&arr_i32) {
        println!("Min of arr_i32: {} at {}", val, idx);
    }
    if let Some((val, idx)) = min_max::find_max(&arr_i32) {
        println!("Max of arr_i32: {} at {}", val, idx);
    }
    
    println!();

    let arr_char = ['k', 'o', 'j', 'b', 'i', 'l', 'e', 'y'];

    println!("arr_char: {:?}", arr_char);

    if let Some((val, idx)) = min_max::find_min(&arr_char) {
        println!("Min of arr_char: {} at {}", val, idx);
    }
    if let Some((val, idx)) = min_max::find_max(&arr_char) {
        println!("Max of arr_char: {} at {}", val, idx);
    }

    println!();

    let arr_f64 = [30.2134f64, 12.0190, 421.2304, 23.42069];

    println!("arr_f64: {:?}", arr_f64);

    if let Some((val, idx)) = min_max::find_min(&arr_f64) {
        println!("Min of arr_f64: {} at {}", val, idx);
    }
    if let Some((val, idx)) = min_max::find_max(&arr_f64) {
        println!("Max of arr_f64: {} at {}", val, idx);
    }

    println!();

    // Edge Case: NaN is always false on comparison, so if it's the first list element,
    // it will result as the "winner" of whatever find.

    #[allow(non_snake_case)]
    let arr_NaN = [f64::NAN, 30.2134, 12.0190, 421.2304, 23.42069];

    println!("arr_NaN: {:?}", arr_NaN);

    if let Some((val, idx)) = min_max::find_min(&arr_NaN) {
        println!("Min of arr_NaN: {} at {}", val, idx);
    }
    if let Some((val, idx)) = min_max::find_max(&arr_NaN) {
        println!("Max of arr_NaN: {} at {}", val, idx);
    }
}
