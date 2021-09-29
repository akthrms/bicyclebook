#[allow(dead_code)]
#[derive(Default)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
    pub stroke_width: u8,
    pub fill: (u8, u8, u8),
    internal_id: String,
}

trait Coordinates {}

#[allow(dead_code)]
#[derive(Default)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

impl Coordinates for CartesianCoord {}

#[allow(dead_code)]
#[derive(Default)]
struct PolarCoord {
    r: f64,
    theta: f64,
}

impl Coordinates for PolarCoord {}

#[allow(unused_variables)]
fn main() {
    let vertexes = vec![
        CartesianCoord { x: 0.0, y: 0.0 },
        CartesianCoord { x: 50.0, y: 0.0 },
        CartesianCoord { x: 30.0, y: 20.0 },
    ];

    let poly = Polygon {
        vertexes,
        ..Default::default()
    };
}
