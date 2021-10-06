use std::fmt;

#[allow(dead_code)]
fn to_n(n: i32) -> impl Iterator {
    0..n
}

// fn to_n_even(n: i32) -> Filter<Range<i32>, fn(&i32) -> bool> {
//     (0..n).filter(|i| i % 2 == 0)
// }

#[allow(dead_code)]
fn to_n_even(n: i32) -> impl Iterator {
    (0..n).filter(|i| i % 2 == 0)
}

fn one() -> impl fmt::Display {
    1i32
}

// fn one(is_float: bool) -> impl fmt::Display {
//     if is_float {
//         1.0f32
//     } else {
//         1i32
//     }
// }

#[allow(dead_code)]
fn get_counter(init: i32) -> impl FnMut() -> i32 {
    let mut n = init;
    move || {
        let ret = n;
        n += 1;
        ret
    }
}

#[allow(unused_variables)]
fn main() {
    let n = one();
    // println!("{}", n + n);
}
