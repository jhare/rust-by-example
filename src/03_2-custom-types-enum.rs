// there are variants of enums/structs that are valid as each other?

// enums specify a custom piece of data with named fields that have a type
enum WebEvent {
    Paste(String),
    KeyPress(char), // they can be like tuple structs
    PageLoad, // an enum can be unit-like, just symbols in a bucket for your use
    PageUnload,
    Click { x: i64, y: i64 }, // they can be c-like structs
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::Paste(s) => println!("pasted is \"{}\"", s),
        WebEvent::KeyPress(some_char) => println!("pressed is '{}'", some_char),

        // unit-like ones just are passed 
        WebEvent::PageLoad => println!("page loaded no params to inject"),
        WebEvent::PageUnload => println!("page unloaded no params here either"),

        // get struct-like params nice way
        WebEvent::Click{ x, y } => println!("x coord is {} y coord is {}", x, y),
    }
}

fn main() {
    println!("example 3-2");

    let pressed = WebEvent::KeyPress('x');

    // create an "owned" string for this WebEvent. I think that makes when "pasted" is cleaned up
    // it makes sure it cleans up the memory for the string we're passing here
    // that's my guess

    let pasted = WebEvent::Paste("my text".to_owned());

    // seems like inspect has the match we need to get inside of enums? can you directly in a
    // println?
    inspect(pressed);
    inspect(pasted);


    let loaded = WebEvent::PageLoad; // yay we don't need () do we
    let unloaded = WebEvent::PageUnload;

    inspect(loaded);
    inspect(unloaded);

    let click = WebEvent::Click{ x: 4, y: 2 };
    inspect(click);
}
