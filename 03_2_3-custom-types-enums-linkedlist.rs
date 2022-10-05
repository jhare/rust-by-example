use crate::List::*;

enum List {
    // a tuple that wraps element and the next in the list. Guess we store u32s.
    Cons(u32, Box<List>), 
    Nil, // the end of the linked list
}

// attaching methods to an enum
impl List {
    // a brand new list just has a value of nil, the end of it
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self)) // Cons will have type List
    }

    // tutorial said this, basically we * the self and have a & because of ownership model
    /**
     *
     *
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail. 
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    */
    fn len(&self) -> u32 {
        match *self { // match the actual type of what's pointed-to
            Cons(_, ref tail) => 1 + tail.len(), // recursive call, 1 + 0 at bottom.
            Nil => 0 // the recurrence bottoms out here
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    println!("Linked list via enums example");

    let mut list = List::new(); // not a new operator, make sure it is 'mut'

    list = list.prepend(42);
    list = list.prepend(69);
    list = list.prepend(256);

    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());
}
