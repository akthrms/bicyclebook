#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() {
    let v1: Vec<u8> = vec![3, 4, 5];

    assert_eq!(v1.first(), Some(&3u8));

    let mut s1 = String::from("Type coercion");
    let s2 = String::from("is actually easy.");

    s1.push_str(&s2);

    let mut i1 = 0u8;
    i1 = 255;

    let p1 = &&&&[1, 2, 3, 4];

    let p2: &[i32; 4] = p1;

    let p3 = &&[1, 2, 3, 4];

    let p4: &[i32; 4] = p3;
    let p5: &[i32] = p4;

    // let p6: &[i32] = p3;
}
