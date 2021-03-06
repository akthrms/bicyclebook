trait As<T> {
    fn cast(self) -> T;
}

impl As<u64> for u8 {
    fn cast(self) -> u64 {
        self as u64
    }
}

impl As<u32> for u8 {
    fn cast(self) -> u32 {
        self as u32
    }
}

#[allow(unused_variables)]
fn main() {
    let one_u32: u32 = 1.cast();
    let one_u64: u64 = 1.cast();
    // let one_i8: i8 = 1.cast();
}
