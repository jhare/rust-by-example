fn doslicethings(someslice: &[i32]) { // do does this mean some reference to an array is automatically a slice ? or what
    println!("First element of slice passed: {}", someslice[0]);
    println!("length of slice {}", someslice.len());
}

fn main() {
    println!("array example");

    // fixed size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Show array of five numbers: {:?}", xs);

    let ys: [i32; 500] = [42; 500]; // init everything to same value
    println!("y after init? {:?}", ys);

    // indexing from 0
    println!("first element {}", xs[0]);
    println!("second element {}", xs[1]);

    // get size
    println!("size of xs {}", xs.len());

    // show size of allocation on stack; arrays "are stack allocated" is that forced?
    println!("array size {} bytes", std::mem::size_of_val(&xs));

    // whole array can be passed as a slice automatically
    doslicethings(&xs);

    for i in 0..xs.len()+1 { // this will go one over
        match xs.get(i) {
            Some(xval) => println!("We have a nice value of {}", xval),
            None => println!("We have gone over with i of {}", i)    
        }
    }

    // we can't do this
    //println!("there is no five {}", xs.5);

}
