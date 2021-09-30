#[allow(unused_variables)]
fn main() {
    let i1 = 42;
    let f1 = i1 as f64;

    let c1 = 'a';
    assert_eq!(c1 as u32, 97);

    let i2 = 300;
    let u1 = i2 as u8;

    assert_eq!(u1, 44);
}
