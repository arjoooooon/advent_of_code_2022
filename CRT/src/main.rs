use std::fs;

#[derive(PartialEq, Eq)]
enum State {
    NOP,
    ADD1,
    ADD2
}

fn part1(lines: &Vec<&str>) -> i32 {
    let mut score_sum: i32 = 0;
   
    let mut register = 1;
    let mut to_add = 0;
    
    let mut idx = 0;
    let mut cycle = 1;

    let mut current_state = match lines[0] {
        "noop" => State::NOP,
        _      => State::ADD1
    };

    let signals = [20, 60, 100, 140, 180, 220]; 

    while idx != lines.len()-1 {
        if signals.contains(&cycle) {
            score_sum += cycle * register;
        }

        if current_state == State::NOP {
            idx += 1;
            current_state = match lines[idx] {
                "noop" => State::NOP,
                _      => State::ADD1
            }
        } else if current_state == State::ADD1 { 
            let tokens: Vec<&str> = lines[idx].split(' ').collect();
            to_add = tokens[1].parse::<i32>().unwrap();
            current_state = State::ADD2;
        } else {
            register += to_add;
            idx += 1;
            current_state = match lines[idx] {
                "noop" => State::NOP,
                _      => State::ADD1
            }
        }

        cycle += 1;
    }
    
    score_sum
}

fn part2(lines: &Vec<&str>) {
    let mut register = 1;
    let mut to_add = 0;
    
    let mut idx = 0;
    let mut cycle = 0;

    let mut current_state = match lines[0] {
        "noop" => State::NOP,
        _      => State::ADD1
    };

    let mut crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];

    while idx != lines.len()-1 {
        // Draw character onto CRT
        let row = cycle / 40;
        let col = cycle % 40;
        if (col as i32 - register).abs() <= 1 {
            crt[row][col] = '#';
        }

        // Process States
        if current_state == State::NOP {
            idx += 1;
            current_state = match lines[idx] {
                "noop" => State::NOP,
                _      => State::ADD1
            }
        } else if current_state == State::ADD1 { 
            let tokens: Vec<&str> = lines[idx].split(' ').collect();
            to_add = tokens[1].parse::<i32>().unwrap();
            current_state = State::ADD2;
        } else {
            register += to_add;
            idx += 1;
            current_state = match lines[idx] {
                "noop" => State::NOP,
                _      => State::ADD1
            }
        }

        cycle += 1;
    }

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", crt[i][j]);
        }
        println!();
    }
    
}

fn main() {
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/CRT/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read file");
    let lines: Vec<&str> = contents.split('\n').collect();

    println!("{}", part1(&lines));
    part2(&lines);
}
