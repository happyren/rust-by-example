#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } } = rect;
    (x2 - x1) * (y1 - y2)
}

fn square(origin: Point, edge: f32) -> Rectangle {
    Rectangle {
        top_left: origin,
        bottom_right: Point { x: origin.x + edge, y: origin.y - edge },
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

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_left_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_left_edge },
        bottom_right: bottom_right,
    };
    
    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let bottom_right = Point { x: 15f32, y: -5.2 };

    let rect = Rectangle {
        top_left: point,
        bottom_right: bottom_right,
    };

    let area = rect_area(rect);
    println!("Area of rectangle is {}", area);

    let new_rect = square(point, 5f32);
    println!("New rectangle is {:?}", new_rect);
}