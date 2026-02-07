mod min_max;

use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    let arr_i32 = [4i32, 8, 22, 94, 44, 55, 92, 83, 17, -23];
    act_on_array(&arr_i32, "arr_i32");

    println!();

    let arr_char = ['k', 'o', 'j', 'b', 'i', 'l', 'e', 'y'];
    act_on_array(&arr_char, "arr_char");

    println!();

    let arr_f64 = [30.2134f64, 12.0190, 421.2304, 23.42069];
    act_on_array(&arr_f64, "arr_f64");

    println!();

    // Edge Case: NaN is always false on comparison, so if it's the first list element,
    // it will result as the "winner" of whatever find.
    #[allow(non_snake_case)]
    let arr_NaN = [f64::NAN, 30.2134, 12.0190, 421.2304, 23.42069];
    act_on_array(&arr_NaN, "arr_NaN");
}

fn act_on_array<T: PartialOrd + Debug + Display>(arr: &[T], name: &str)
where T: PartialOrd + Debug + Display, 
{
    println!("{}: {:?}", name, arr);

    if let Some((val, idx)) = min_max::find_min(&arr) {
        println!("Min of {}: {} at {}", name, val, idx);
    }

    if let Some((val, idx)) = min_max::find_max(&arr) {
        println!("Max of {}: {} at {}", name, val, idx);
    }
}
