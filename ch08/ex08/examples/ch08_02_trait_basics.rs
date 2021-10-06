struct CartesianCoord {
    x: f64,
    y: f64,
}

struct PolarCoord {
    r: f64,
    theta: f64,
}

trait Coordinates {
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

// fn print_point<P: Coordinates>(point: P) {
//     let p = point.to_cartesian();
//     println!("({}, {})", p.x, p.y);
// }

// fn print_point<P>(point: P)
// where
//     P: Coordinates,
// {
//     let p = point.to_cartesian();
//     println!("({}, {})", p.x, p.y);
// }

fn print_point(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

#[allow(dead_code)]
fn as_cartesian<P: Coordinates + Clone>(point: &P) -> CartesianCoord {
    point.clone().to_cartesian()
}

#[allow(dead_code)]
fn double_point<P: Coordinates>(point: P) -> P {
    let mut cart = point.to_cartesian();
    cart.x *= 2.0;
    cart.y *= 2.0;
    P::from_cartesian(cart)
}

#[allow(dead_code)]
fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

#[allow(dead_code)]
fn to<T>(i: i32) -> T
where
    i32: ConvertTo<T>,
{
    i.convert()
}

fn main() {
    print_point((0.0, 1.0));
    print_point(PolarCoord {
        r: 1.0,
        theta: std::f64::consts::PI,
    });

    // print_point("string");
}
