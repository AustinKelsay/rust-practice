use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

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
    // let contents = format!("{}\t{}\n", key, value);
    // A result is rusts way of handling errors
    // Since rust cant do 'exceptions' in the language, we use Result to at least see what is the result from the command that crashed our program
    // fs::write("kv.db", contents);

    let mut database = Database::spawn().expect("Failed to spawn database");
    // in rust any param passed into a func is moved into the func 'passed by value' by default
    // if you want to pass by refference use & or .clone()
    database.add_data(key.to_uppercase(), value.clone());
    database.add_data(key, value);
    database.flush().unwrap()
}

// Rust is not OOP, it is functional so we dont have classes
// How we work with class like functionality is to define a struct first that just houses field names/types as KV pairs
struct Database {
    map: HashMap<String, String>,
}

// If we want "methods" in our struct, we need to define them in an impl block
impl Database {
    // First check if the kv.db file exists
    // let db_exists = Path::new("kv.db").exists();
    // Result<T, E> is the type used for returning and propagating errors. It is an enum with the variants, Ok(T), representing success and containing a value, and Err(E), representing error and containing an error value.
    fn spawn() -> Result<Database, std::io::Error> {
        // Read kv.db file
        // let contents: String = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => {
        //         return c
        //     }
        //     Err(e) => {
        //         println!("Error reading file: {:?}", e);
        //         return Err(e)
        //     }
        // };
        let mut map = HashMap::new();
        // This syntax below is equivalent to the above pattern
        // The question mark is an aknowledgement that the function will return an error if it fails
        if !Path::new("kv.db").exists() {
            File::create("kv.db")?;
        }
        let contents = std::fs::read_to_string("kv.db")?;
        // Split the contents into lines and iterate
        for line in contents.lines() {
            // Split our line into two chunks
            let mut chunks = line.splitn(2,'\t');
            let key = chunks.next().expect("No key found");
            let value = chunks.next().expect("No value found");
            // Pass ownership of the key and value so we are passing a <String> and not a &str which is just a reference or pointer to that actual value
            map.insert(key.to_owned(), value.to_owned());
        }
        // parse the string
        // populate our map
        Ok(Database {
            map: map,
        })
    }

    fn add_data(&mut self, key: String, value: String) {
        // Insert the key and value into our map
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        // SUMMARY: Prefer &str as a function parameter or if you want a read-only view of a string; String when you want to own and mutate a string.
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n')
        }
        std::fs::write("kv.db", contents)
    }
}