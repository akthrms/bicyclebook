// fn f1(name: &str) -> &str {
//     let s = format!("Hello, {}!", name);
//     &s
// }

fn f1(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    f1("ken");
}
