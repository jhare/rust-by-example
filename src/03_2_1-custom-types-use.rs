#![allow(dead_code)]
enum Status {
    Poor,
    Rich,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // we can apply the 'use' syntax to bring enum symbols into the scope of a function.
    use crate::Status::{Poor, Rich};

    use crate::Work::*;

    let status = Poor; // equiv to `Status::Poor`, didn't have to scope it here

    let work = Civilian;

    // no scoping needed like in the WebEvent example
    match status {
        Rich => println!("the rich have the paper"),
        Poor => println!("the poor hand it over"),
    }

    match work {
        Civilian => println!("civilians work"),
        Soldier => println!("soldiers fight"),
    }
}
