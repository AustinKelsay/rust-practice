fn types() {
    // compound types
    // compund types are types that contain other types
    let tup: (i32, f64) = (500, 6.4);
    // destructuring
    let (x, y) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}