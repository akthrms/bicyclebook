#[allow(dead_code)]
// #[derive(Default)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1,
            vertexes: Default::default(),
            fill: Default::default(),
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let polygon1 = Polygon::default();

    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        ..Default::default()
    };
}
