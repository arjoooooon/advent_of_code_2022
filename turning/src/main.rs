use std::fs;
use std::collections::HashMap;

fn main() {
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/turning/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read file");

    let mut set: HashMap<char,i32> = HashMap::new(); 
    let byte_string = contents.as_bytes();

    for i in 0..14 {
        let ch = byte_string[i] as char;
        if set.contains_key(&ch) {
            *set.get_mut(&ch).unwrap() += 1;
        } else {
            set.insert(ch, 1);
        }

    }
    if set.len() == 14 {
        println!("4");
        return;
    } 

    for i in 14..byte_string.len() {
        let prev: char = byte_string[i-14] as char;
        let new: char = byte_string[i] as char;

        *set.get_mut(&prev).unwrap() -= 1;
        if *set.get(&prev).unwrap() == 0 {
            set.remove(&prev);
        }

        if set.contains_key(&new) {
            *set.get_mut(&new).unwrap() += 1;
        } else {
            set.insert(new, 1);
        }
        
        if set.len() == 14 {
            println!("{}", i+1);
            return;
        }
    }

    return;



}
