fn main() {
    println!("exercise 8-5 flow control: match");
    let number = 13;

    // pattern matching
    match number {
        1 => println!("One!"),

        2 | 3 | 5 | 7 | 11 | 13 => println!("a prime!"), // stops here; "short circuit" like a switch

        13..=19 => println!("in the teens"),

        _ => println!("everything else case"),
    }
}
