#![allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,

    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    println!("example 8-5-1-3 flow control: match destructuring enums");

    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),

        Color::RGB(r, g, b) => println!("red {} green {} blue {}", r, g, b),
        Color::HSV(h, s, v) => println!("hue {} saturation {} value {}", h, s, v),
        Color::HSL(h, s, l) => println!("hue {} saturation {} lightness {}", h, s, l),
        Color::CMY(c, m, y) => println!("cyan {} magenta {} yellow {}", c, m , y),
        Color::CMYK(c, m, y, k) => println!("cyan {} magenta {} yellow {} key (black) {}", c, m, y , k),
    }
}
