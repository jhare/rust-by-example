fn main() {
    println!("example 8-5-1-5 flow control: match destructuring structs");

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // trying to change struct vals...
    let foo = Foo { x: (1, 2), y: 3};

    match foo {
        Foo { x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),

        // destructure the struct into i
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // ignore variables
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}
