#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit; // a unit struct; Is that like "unit type"?

struct Pair(i32, f32); // a "tuple struct" is what they call this, not anonymous.

// struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// structs can be fields in structs
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    println!("exercise 3-1");

    let name = String::from("Peter"); // neat way to make a String-class string
    let age = 27;

    let peter = Person { name, age };

    // debug-print a struct
    println!("person is {:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coords: ({}, {})", point.x, point.y);

    // use the .. syntax to fill the 'y' field with point 'struct update syntax'
    let bottom_right = Point { x: 5.2, ..point } ; // use the value for 'y' of variable 'point'

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the 'point' variable into two new vars with a let syntax
    let Point { x: left_edge, y: top_edge } = point;
    // this declares left_edge and top_edge, gives them the values of x and y from 'point',
    // respectively. Obviously we have to tell it which fields destructure where. It looks almost
    // like we're declaring a new point but that's not the case I think.

    let _rectangle = Rectangle {
        // this is 'struct instantiation'.
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // make a unit struct
    let _unit = Unit;

    // make a tuple struct
    let pair = Pair(1, 0.1);

    // access fields of that tuple struct
    println!("fields of tuple {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
