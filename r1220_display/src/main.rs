use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
static X: Structure = Structure(3);

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real: {}, imag: {}", self.real, self.imag)
    }
}
fn main() {
    println!("{}", X);
    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );
    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("{}", complex);
    println!("{:?}", complex)
}
