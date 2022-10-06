fn main() {
    println!("exercise 8-1 flow control: if/else");

    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold pls");
            // remember NO semicolon here. I bet a lot of people fuck this up
            10 * n 
        } else {
            println!(", and is a big number, half the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);
}
