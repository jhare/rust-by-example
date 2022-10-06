#![allow(overflowing_literals)]

fn main() {
    println!("exercise 5-1 types: casting");

    let decimal = 65.4321_f32; // nice, suffix annotation using _ to separate.

    // error can't do this
    // let integer: u8 = decimal; // truncation

    // explicit casts
    let integer = decimal as u8;
    let character = integer as char; 

    // error: float cannot be converted to char
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // lsb half is kept, msb truncated
    println!("1000 as u8 is : {}", 1000 as u8);

    // -1 + 256 = 255
    println!("-1 as u8: {}", (-1i8) as u8);

    // same as modulus > 0 
    println!("1000 mod 256 is: {}", 1000 % 256);
    
    // may already fit
    println!("128 as a i16 is {}", 128 as i16);

    // -128 in 2's complement is...
    println!("128 in 2's complement is {}", 128 as i8);

    // from above examples
    println!("1000 as a u8 is: {}", 1000 as u8);
    println!("232 as i8 is {}", 232 as i8);

    // truncating f32 to u8
    println!("300.0 is {}", 300.0_f32 as u8);

    println!("-100.0 as u8 is {}", -100.0_f32 as u8);

    println!("nan as u8 is {}", f32::NAN as u8);

    // these methods aren't great, pasted them in here for posterity.

    // This behavior incurs a small runtime cost and can be avoided 
    // with unsafe methods, however the results might overflow and 
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
