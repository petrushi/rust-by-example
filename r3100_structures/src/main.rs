use std::u8;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(u32, f32);

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
//activity
fn rect_area(rectangle: Rectangle) -> f32 {
    let width = rectangle.bottom_right.x - rectangle.top_left.x;
    let height = rectangle.top_left.y - rectangle.bottom_right.y;
    return width * height;
}

fn square(point: Point, size: f32) -> Rectangle {
    let bottom_right = Point {
        x: point.x + size,
        y: point.y,
    };
    let top_left = Point {
        x: point.x,
        y: point.y + size,
    };
    Rectangle {
        top_left: top_left,
        bottom_right: bottom_right,
    }
}
fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point is: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    println!("top edge and left edge are: ({}, {})", top_edge, left_edge);

    let one_rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    let second_rectangle = Rectangle {
        top_left: Point { x: 1.0, y: 3.0 },
        bottom_right: Point { x: 4.0, y: 1.0 },
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("area of rectangle is {}", rect_area(one_rectangle));
    println!(
        "area of second rectangle is {}",
        rect_area(second_rectangle)
    );
    let start_point = Point { x: 2.0, y: 2.0 };
    let third_rectangle = square(start_point, 10.0);
    println!(
        "points of rectangle are {:?} {:?} {:?} {:?}",
        third_rectangle.top_left.x,
        third_rectangle.bottom_right.x,
        third_rectangle.bottom_right.y,
        third_rectangle.top_left.y
    );
}
