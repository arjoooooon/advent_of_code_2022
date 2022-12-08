use std::fs;

fn main() {
    let filepath: &str = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code/rps/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read file to string");

    let mut total: i32 = 0;

    for string in contents.split('\n') {
        if string.is_empty() {
            continue;
        }

        let bytes = string.as_bytes();
        let p1: char = bytes[0] as char;
        let p2: char = bytes[2] as char;

        let mut val1: i32 = 0;

        match p1 {
            'A' => val1 = 0,
            'B' => val1 = 1,
            'C' => val1 = 2,
            _ => println!("Something is wrong with the data")
        };
        
        let mut score: i32 = 0;

        match p2 {
            'X' => score = 1+(val1-1)%3,
            'Y' => score = 4+val1,
            'Z' => score = 7+(val1+1)%3,
            _ => println!("Something is wrong with the data")
        };

        total += score;
    }

    println!("{}", total);
    return;
        
}

