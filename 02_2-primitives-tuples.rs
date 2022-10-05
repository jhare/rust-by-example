fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_val, bool_val) = pair;
    return (bool_val, int_val);
}

fn main() {
    let my_pair = (42, false);

    println!("My pair is {:?}", my_pair);
    println!("When reversed is {:?}", reverse(my_pair));

    // can be all different things
    let long_tuple = (1u8, 2u16, 3u32, 4);
}
