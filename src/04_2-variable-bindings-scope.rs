fn main() {
    println!("exercise 4-1 variable bindings: scope");

    let long_lived_binding = 1;

    let shadowed_binding = 42;

    // create a new arbitrary block
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let shadowed_binding = "foo";

        println!("we have shadowed variable {}", shadowed_binding);
    }

    // println!("the inner short isn't available {}", short_lived_binding); // un commenting makes
    // an error here.

    println!("the outer was always there {}", long_lived_binding);
}
