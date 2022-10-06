fn main() {
    println!("exercise 8-4 flow control: for loops");

    // .. itself is (inclusive)..(exclusive)
    for n in 1..101 {
        // we want n to hit n=100
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // they call this range
    // this ..= construct (new to me) is (inclusive)..=(inclusive)
    fn main() {
        for n in 1..=100 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }
    }
}
