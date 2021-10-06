#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct CartesianCoord {
    pub x: f64,
    pub y: f64,
}

pub struct PolarCoord {
    pub r: f64,
    pub theta: f64,
}

pub trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;

    fn from_cartesian(cart: CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn print_point(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

struct Matrix([[f64; 2]; 2]);

trait LinearTransform: Coordinates {
    fn transform(self, matrix: &Matrix) -> Self
    where
        Self: Sized,
    {
        let mut cart = self.to_cartesian();
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;

        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * x + m[1][1] * y;
        Self::from_cartesian(cart)
    }

    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }
}

impl LinearTransform for CartesianCoord {}

impl LinearTransform for PolarCoord {
    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}

fn main() {
    let p = (1.0, 0.0).to_cartesian();
    print_point(p.rotate(std::f64::consts::PI));
}
