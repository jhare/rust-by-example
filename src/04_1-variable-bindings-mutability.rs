fn main() {
    println!("exercise 4-1 variable bindings: mutability");

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mtuation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    // Error!
    // _immutable_binding += 1; // when uncommented this breaks compilation
}
