fn main() {
    println!("exercise 5-3 types: inference");

    // explicit annotation
    let elem = 5u8;

    // create empty vector
    let mut vec = Vec::new(); // compiler doesn't know yet

    vec.push(elem); // this tell's comnpiler vec is a Vec<u8>

    println!("{:?}", vec);
}
