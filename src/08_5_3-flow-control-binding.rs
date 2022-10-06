#![allow(dead_code)]

// implicit u32 return in the value
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("example 8-5-3 flow control: binding");

    match age() {
        0 => println!("age is zero"),
        n @ 1 ..= 12 => println!("I'm child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teenager of age {:?}", n),
        n => println!("Some other age {:?}", n),
    }

    match some_number() {
        Some(n @ 42) => println!("The Answer: {}", n),
        Some(n) => println!("something that is not the answer... {}", n),
        _ => (),
    }
}
