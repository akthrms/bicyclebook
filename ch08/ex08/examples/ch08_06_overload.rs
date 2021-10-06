// trait Overload {
//     fn call(&self) -> &'static str;
// }

// impl Overload for i32 {
//     fn call(&self) -> &'static str {
//         "i32"
//     }
// }

// impl Overload for str {
//     fn call(&self) -> &'static str {
//         "str"
//     }
// }

trait Overload<T> {
    fn call(&self, t: T) -> &'static str;
}

impl Overload<i32> for i32 {
    fn call(&self, _: i32) -> &'static str {
        "(i32, i32)"
    }
}

impl Overload<char> for i32 {
    fn call(&self, _: char) -> &'static str {
        "(i32, char)"
    }
}

fn main() {
    // assert_eq!(1i32.call(), "i32");
    // assert_eq!("str".call(), "str");

    // assert_eq!(Overload::call(&1i32), "i32");
    // assert_eq!(Overload::call("str"), "str");

    assert_eq!(1i32.call(2i32), "(i32, i32)");
    assert_eq!(1i32.call('c'), "(i32, char)");
}
