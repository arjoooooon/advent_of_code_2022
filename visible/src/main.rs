use std::fs;
use std::cmp::max;

fn part1() -> i32{
    // Set up lines
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/visible/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read from file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let rows = lines.len()-1;
    let cols = lines[0].len();
    
    let mut grid: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let ch = lines[i].as_bytes()[j];
            let val = (ch as char).to_digit(10).unwrap();
            grid[i][j] = val as i32;
        }
    }

    let mut processed = vec![vec![false; cols]; rows];
   
    // Set the outer grid to be true
    for row in 0..rows {
        processed[row][0] = true;
        processed[row][cols-1] = true;
    }

    for col in 0..cols {
        processed[0][col] = true;
        processed[rows-1][col] = true;
    }

    // Start left scan
    for row in 0..rows {
        let mut left_max = grid[row][0];
        for col in 1..cols {
            if grid[row][col] > left_max {
                processed[row][col] = true;
            }
            left_max = max(grid[row][col], left_max);
        }
    } 

    // Start top scan
    for col in 0..cols {
        let mut top_max = grid[0][col];
        for row in 1..rows {
            if grid[row][col] > top_max {
                processed[row][col] = true;
            }
            top_max = max(grid[row][col], top_max);
        }
    } 

    // Start right scan
    for row in 0..rows {
        let mut right_max = grid[row][cols-1];
        for col in (0..cols-1).rev() {
            if grid[row][col] > right_max {
                processed[row][col] = true;
            }
            right_max = max(grid[row][col], right_max);
        }
    } 

    // Start bottom scan
    for col in 0..cols {
        let mut bottom_max = grid[rows-1][col];
        for row in (0..rows-1).rev() {
            if grid[row][col] > bottom_max {
                processed[row][col] = true;
            }
            bottom_max = max(grid[row][col], bottom_max);
        }
    } 
    
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if processed[row][col] {
                 count += 1;
            }
        }
    }

    count
}

#[allow(dead_code)]
fn part2() -> i32 {
    // Set up lines
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/visible/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read from file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let rows = lines.len()-1;
    let cols = lines[0].len();

    let mut grid: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let ch = lines[i].as_bytes()[j];
            let val = (ch as char).to_digit(10).unwrap();
            grid[i][j] = val as i32;
        }
    }
        
    let mut max_score: i32 = 0;
    for row in 2..rows {
        for col in 2..cols {
            let mut score = 1;

            // Probe left
            let mut left_max = col;
            for i in (0..col).rev() {
                left_max = i;
                if grid[row][left_max] >= grid[row][col] {
                    break;
                } 
            }
            score *= col - left_max;

            // Probe up
            let mut top_max = row;
            for i in (0..row).rev() {
                top_max = i; 
                if grid[top_max][col] >= grid[row][col] {
                    break;
                }
            }
            score *= row - top_max;

            // Probe right
            let mut right_max = col;
            for i in col+1..cols {
                right_max = i; 
                if grid[row][right_max] >= grid[row][col] {
                    break;
                }
            }
            score *= right_max - col;

            // Probe down
            let mut bottom_max = row;
            for i in row+1..rows {
                bottom_max = i; 
                if grid[bottom_max][col] >= grid[row][col] {
                    break;
                }
            }
            score *= bottom_max - row;

            max_score = max(score.try_into().unwrap(), max_score);
        }
    }


    max_score
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
