fn main() {
    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];

    let largest: i32 = get_largest(num_list);
    println!("The largest number is {}", largest);

    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let largest_char: char = get_largest_char(char_list);
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