use std::fmt::Display;
use std::string::ToString;

#[allow(dead_code)]
fn stringify(t: Box<dyn ToString>) -> String {
    t.to_string()
}

fn main() {
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1i32);
}
