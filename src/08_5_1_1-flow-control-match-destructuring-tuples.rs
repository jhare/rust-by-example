// these example outlines are getting silly.... keeping with it.
fn main() {
    println!("exercise 8-5-1-1 flow-control: destructuring tuples");
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    // vim could use better highlighting of this word.
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("Start 3 end 4"),
        _ => println!("It doesn't matter what they are"),
    }
}
