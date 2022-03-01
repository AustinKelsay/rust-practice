use std::fs;

fn main() {
    // Grab CLI args
    // args is an iterator
    // We want to skip the first arg, which is the name of the program
    let mut arguments = std::env::args().skip(1);
    // Define the key in our K/V store
    // type: Option<String> means that the value can be a string or none
    let key = arguments.next().expect("No key specified");
    let value = arguments.next().expect("No value specified");
    println!("The key is '{}', and the value is {}", key, value);
    // Define the format of our output file
    let contents = format!("{}\t{}\n", key, value);
    // A result is rusts way of handling errors
    // Since rust cant do 'exceptions' in the language, we use Result to at least see what is the result from the command that crashed our program
    let write_result = fs::write("test.txt", contents);
    // Match lets us do a kind of try except but it is not based on errors rather it is based on the result of the command
    match write_result {
        Ok(()) => {
            println!("Successfully wrote to the file");
        }
        Err(e) => {
            println!("Error writing to file: {:?}", e);
        }
    }
}
