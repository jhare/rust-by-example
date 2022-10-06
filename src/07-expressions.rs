fn main() {
    println!("exercise 7-1 expressions");
    let x = 5u32;

    x;
    x+1;
    15;

    let y = {
        let x_squared = x+x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x; // semicolon keeps this value from being assigned to z makes it () unit
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

