fn main() {
    println!("example 8-5-1-2 flow control: match destructuring arrays and slices");
    let array = [1, -4, 6]; // this can be a slice, too.

    match array  {
        // bind to variables "second" and "third"
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // single values can be ignored using _ pattern
        [1, _, third] => println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),

        // filter by -1, capture value in "second" variable, ignore rest
        [-1, second, ..] => println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),

        // code below won't compile?
        // [-1, second] => ...

        // store in another slice named "tail" by us
        [3, second, tail @ .. ] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),

        // we can package up the whole thing kind of like a pumping lemma
        [first, middle @ .., last] => println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }
}
