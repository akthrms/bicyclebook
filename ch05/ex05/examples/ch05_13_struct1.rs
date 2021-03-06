#[allow(dead_code)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon {
        vertexes,
        stroke_width,
        fill,
    }
}

#[allow(unused_variables)]
fn main() {
    let triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));

    let Polygon {
        vertexes: quad_vx, ..
    } = quadrangle;

    assert_eq!(quad_vx.len(), 4);

    let Polygon { fill, .. } = quadrangle;

    assert_eq!(fill, (0, 0, 0));

    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };

    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
        ..triangle1
    };
}
