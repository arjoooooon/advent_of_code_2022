use std::fs;

fn main() {
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/cleanup/data/data.txt";

    let contents = fs::read_to_string(filepath)
        .expect("Failed to read from input file");
    
    let mut count = 0;
    for pair in contents.split('\n') {
        if pair.is_empty() {
            continue;
        }
        let values: Vec<&str> = pair.split(',').collect();

        let range1: Vec<&str> = values[0].split('-').collect();
        let range2: Vec<&str> = values[1].split('-').collect();
        
        let i11 = range1[0].parse::<i32>().unwrap();
        let i12 = range1[1].parse::<i32>().unwrap();
        let i21 = range2[0].parse::<i32>().unwrap();
        let i22 = range2[1].parse::<i32>().unwrap();
            
        // Check if either is within the bounds of the other
        if i21 >= i11 && i21 <= i12 {
            count += 1;
        }  else if i22 >= i11 && i22 <= i12 {
            count += 1;
        } else if i11 >= i21 && i11 <= i22 {
            count += 1;
        } else if i12 >= i21 && i12 <= i22 {
            count += 1;
        }

    }

    println!("{}", count);
}
