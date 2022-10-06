fn main() {
    println!("exercise 8-2-2 flow control: returning from loops");

    let mut counter = 0;

    // sometimes we want to retry something until success
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // break works like a return for the loop closure.
        }
    };

    assert_eq!(result, 20);
    println!("result is {} and value was {}", result, 20);
}
