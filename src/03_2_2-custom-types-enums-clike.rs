#![allow(dead_code)]

// enums will number themselves like C ones
enum Number {
    Zero, // implicit discriminator starts at 0. almost its value, that is
    One,
    Two
}

// you can assign explicit int values i32
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
