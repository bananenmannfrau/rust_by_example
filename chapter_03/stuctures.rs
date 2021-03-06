use std::f32;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}


fn react_area(rect: Rectangle) -> f32 {
    let Rectangle{p1: Point{x: x1,y: y1},p2:  Point{x: x2,y: y2}} = rect;
    (((x1-x2).powf(2.0)) +  ((y1-y2).powf(2.0))).sqrt()
}

fn square(p: Point, width: f32) -> Rectangle {
    let Point{x: x1,y: y1} = p;
    let new_point = Point{x: x1+width,y: y1};
    Rectangle{p1: p, p2: new_point}
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    
    let p3 = Point{x: 1f32, y: 2f32};

    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("area of rect = {}", react_area(_rectangle));
    println!("new react = {:?}", square(p3, 1.5));
}

