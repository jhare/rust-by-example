// adding comment to see branch change
fn main() {
    let a_binding;

    {
        let x = 2;
        let a = 4;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    //println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another is: {}", another_binding);
}
