fn main() {
    let mut an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy an into 'copied'
    let copied_integer = an_integer;

    println!("Copied is {}", copied_integer);

    // change the source, should NOT change copy
    an_integer = 42; // can't do this witout mut

    println!("Original is {} but copy is still {}", an_integer, copied_integer);

    println!("The 'unit' value: {:?}", unit);

    let _unused_variable = 3u32; // compiler skips warning if variable name starts with _



}
