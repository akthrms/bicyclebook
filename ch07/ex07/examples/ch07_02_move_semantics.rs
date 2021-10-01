use std::ops::Drop;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1);

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);
}

fn _f1(p: &Parent) {
    println!("p: {:?}", p);
}

fn _f2(p: &mut Parent) {
    p.0 *= 1;
}

fn _borrow() {
    let mut p1 = Parent(1, Child(11), Child(12));
    _f1(&p1);
    _f2(&mut p1);
    println!("p1: {:?}", p1);
}
