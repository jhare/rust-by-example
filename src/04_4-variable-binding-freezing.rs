fn main() {
    println!("exercise 4-4 variable binding: freeze");

    let mut _mutable_integer = 7i32;

    {

        // shadowing by immutable
        let _mutable_integer = _mutable_integer;

        // can't do this
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
