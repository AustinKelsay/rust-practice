use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess a number between 1-100!");
    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    loop {
        let mut guess: String = String::new();
        
        io::stdin() 
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // The trim() method removes any whitespace from the beginning and end of the string.
            // The parse() method converts the string to a number.
            // The expect() method returns an error if the string can’t be converted to a number.
            // The msg!() macro is a shortcut for creating a string literal.
            // The msg!() macro is a shortcut for creating a string literal.
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break; // break out of the loop
            }
        }
    }
}
