fn main() {
    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];

    // let largest: i32 = get_largest(num_list);
    let largest = get_largest_generic(num_list);
    println!("The largest number is {}", largest);

    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    
    let largest_char = get_largest_generic(char_list);
    // let largest_char: char = get_largest_char(char_list);
    println!("The largest char is {}", largest_char);
}

fn get_largest(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn get_largest_char(list: Vec<char>) -> char {
    let mut largest = list[0];

    for character in list {
        if character > largest {
            largest = character;
        }
    }

    largest
}

// Using genric type
// We use the <T> syntax to specify the type of the generic parameter.
// We can pass traits into the generic type to assert
// below we are asserting that the type T can be ordered and it is a type that can be copied
fn get_largest_generic<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}