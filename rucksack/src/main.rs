use std::fs;
use std::collections::HashSet;

fn priority(ch: char) -> i32 {
    let mut ans: i32 = 0;

    if ch >= 'a' && ch <= 'z' {
        ans =  1 + ch as i32 - 'a' as i32;
    } else if ch >= 'A' && ch <= 'Z' {
        ans =  27 + ch as i32 - 'A' as i32;
    }

    return ans;
}

fn main() {
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code/rucksack/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read file");
    
    // FIRST PART
    /*
    for string in contents.split('\n') {
        let size: usize = string.len();
        let mut set = HashSet::new();

        for ch in string[..size/2].chars() {
            // Add items to Hashset
            set.insert(ch); 
        } 

        for ch in string[size/2..].chars() {
            // Check if a string is in the previous set
            if set.contains(&ch) {
                cost += priority(ch);
                break;
            }
        }
    }
    */

    let split = contents.split('\n');
    let strings: Vec<&str> = split.collect();
    let size: usize = strings.len();
    let mut total: i32 = 0;

    for i in 0..size/3 {
        let str1 = strings[3*i];
        let str2 = strings[3*i+1];
        let str3 = strings[3*i+2];

        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut set3 = HashSet::new();

        for ch in str1.chars() {
            set1.insert(ch);
        }
        for ch in str2.chars() {
            set2.insert(ch);
        }
        for ch in str3.chars() {
            set3.insert(ch); 
        }

        let is1: HashSet<_> = set1.intersection(&set2).copied().collect();
        let is2: HashSet<_> = is1.intersection(&set3).collect();

        for key in is2 {
            total += priority(*key);
        }

    }

    println!("{}", total);
}
