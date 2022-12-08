use std::fs;
use std::env;
use std::collections::BinaryHeap;

fn main() {
    let args: Vec<String> = env::args().collect();    

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read file at given filepath");

    let cal_iter = contents.split('\n');

    let mut current_ = 0;
    let mut heap = BinaryHeap::new();

    for calories in cal_iter {
        if calories.is_empty() {
            // Reset max_ and current
            heap.push(current_);
            current_ = 0;
        } else {
            // Parse int and add to current
            let value: u32 = calories.parse::<u32>().unwrap();
            current_ += value;
        }
    }
    
    heap.push(current_);

    let mut total: u32 = 0;

    total += heap.pop().unwrap();
    total += heap.pop().unwrap();
    total += heap.pop().unwrap();
    
    println!("{}", total);

    return;
}
