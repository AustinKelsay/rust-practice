fn main() {
    let mut x = 5;
    println!("x has the value {}", x);
    x = 6;
    println!("x has the value {}", x);

    // const cant not be mutated
    const MAX_POINTS: u32 = 1000;
    // shadowing
    // creates a new variable with a new spot in memory. The name “my_num” now refers to the new variable, and the old variable is no longer accessible by its name.
    let my_num = 5;
    let my_num = 6;
    println!("{}", my_num);
    println!("{}", sum(3, 4));
}

// Use arrow to denote the return type
fn sum(x: i32, y: i32) -> i32 {
    return x + y
}