fn main() {
    println!("exercise 8-2 flow control loop");
    let mut count = 0u32;

    // you can make infinite loops, decided when to continue/break;
    loop {
        count += 1;

        if count == 1 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Enough for me!");
            break;
        }
    }
}
