#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

trait Dimension {
    const DIMENSION: u32;
}

impl Dimension for CartesianCoord {
    const DIMENSION: u32 = 2;
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let dim = CartesianCoord::DIMENSION;

    const DIM: u32 = CartesianCoord::DIMENSION;
}
