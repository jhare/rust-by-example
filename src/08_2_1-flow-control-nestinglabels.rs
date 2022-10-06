#![allow(unreachable_code)]
fn main() {
    println!("exercise 8-2-1 flow control: nesting & labels");

    'outer: loop {
        println!("Entered outer loop");

        'inner: loop {
            println!("entered inner loop");
            // break; implicit breaks do 'inner here

            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited outer loop");
}
