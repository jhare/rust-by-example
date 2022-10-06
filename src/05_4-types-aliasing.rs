type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    println!("exercise 5-4 types: aliasing");

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // there is no type safety between these, you can add them all up even if
    // the names you picked mean they "shouldn't"
    println!("{} nanoseconds + {} inches = {} units",
        nanoseconds,
        inches,
        nanoseconds + inches); // these added up don't really make sense, we just gave them names arbitrarily.
}
