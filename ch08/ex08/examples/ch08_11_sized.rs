use std::mem::size_of;

fn main() {
    println!("{}", size_of::<&i32>());
    println!("{}", size_of::<&str>());
}
