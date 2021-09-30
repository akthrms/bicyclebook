fn f1(p: &[i32]) -> i32 {
    p[0]
}

fn f2(p: Box<[i32]>) -> i32 {
    p[0]
}

fn main() {
    let a1 = [1, 2, 3, 4];
    assert_eq!(f1(&a1), 1);
    assert_eq!(f2(Box::new(a1)), 1);

    let mut _d: Box<dyn std::fmt::Debug>;

    _d = Box::new([1, 2]);
    _d = Box::new(Some(1));
}
