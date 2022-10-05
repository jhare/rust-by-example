fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_val, bool_val) = pair;
    return (bool_val, int_val);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let my_pair = (42, false);

    println!("My pair is {:?}", my_pair);
    println!("When reversed is {:?}", reverse(my_pair));

    // can be all different things
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("Long tuple 2nd value: {}", long_tuple.1);
    println!("Long tuple 5th value: {}", long_tuple.4);

    // can have tuples of tuples
    let tuple_of_tuples = (("can", "have"), ("tuples", 42, "in"), "tuples");

    println!("looking at tuple of tuples {:?}", tuple_of_tuples);

    let some_tuple = ("withatrailingcomma",);

    println!("what dimension of tuple do we have {:?}", some_tuple);

    println!("what is at zero {}", some_tuple.0);
    // println!("what is at one {}", some_tuple.1); this creates an error, even though the debug
    // output has the comma in it with no member after, guess somehow after is some known empty?

   
    // tuples support destructuring
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("Parts are {:?} {:?} {:?} {:?}", a, b, c, d);

    // An anonymous struct can be defined with a tuple looks like
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix is {:?}", matrix);
}
