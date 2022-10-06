fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed get their types inferred by the compiler looking ahead for you.
    let i = 1; // default to i32?
    let f = 1.0; // default to f64?

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}
