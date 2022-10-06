// these example outlines are getting silly.... keeping with it.
fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        _ => println!("It doesn't matter what they are"),
    }
}
