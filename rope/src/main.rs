use std::fs;
use std::collections::HashSet;

#[allow(dead_code)]
fn part1(lines: &Vec<&str>) -> usize {
    let mut ref_map: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    ref_map.insert(tail);

    for line in lines {
        if *line == "" {
            break;
        }
        let tokens: Vec<&str> = line.split(' ').collect();

        let mov = match tokens[0] {
            "L" => (-1,  0),
            "U" => ( 0,  1),
            "R" => ( 1,  0),
            "D" => ( 0, -1),
            _ => head
        };
        let count = tokens[1].parse::<i32>().unwrap();

        for _ in 0..count {
            // Move the head
            head = (mov.0 + head.0, mov.1 + head.1);

            // Now we process where the tail goes
            let x_diff = head.0 - tail.0;
            let y_diff = head.1 - tail.1;

            // Up right
            if (x_diff > 1 && y_diff > 0) || (x_diff > 0 && y_diff > 1) {
                tail = (tail.0+1, tail.1+1);
            } else if (x_diff > 1 && y_diff < 0) || (x_diff > 0 && y_diff < -1) {
                tail = (tail.0+1, tail.1-1);
            } else if (x_diff < 0 && y_diff < -1) || (x_diff < -1 && y_diff < 0) {
                tail = (tail.0-1, tail.1-1);
            } else if (x_diff < 0 && y_diff > 1) || (x_diff < -1 && y_diff > 0) {
                tail = (tail.0-1, tail.1+1);
            } else if x_diff > 1 {
                tail = (tail.0+1, tail.1);
            } else if x_diff < -1 {
                tail = (tail.0-1, tail.1);
            } else if y_diff > 1 {
                tail = (tail.0, tail.1+1);
            } else if y_diff < -1 {
                tail = (tail.0, tail.1-1);
            }

            ref_map.insert(tail);
            // println!("{:?} -> {:?}", head, tail);
        }
    }

    return ref_map.len();
}

fn part2(lines: &Vec<&str>) -> usize {
    let mut knots = vec![(0, 0); 10]; 
    let mut ref_map: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        if *line == "" { break; }
        let tokens: Vec<&str> = line.split(' ').collect();
       
        let mov = match tokens[0] {
            "L" => (-1,  0),
            "U" => ( 0,  1),
            "R" => ( 1,  0),
            "D" => ( 0, -1),
            _ => break
        };
        
        let count: usize = tokens[1].parse::<usize>().unwrap();
        for _ in 0..count { 
            knots[0] = (knots[0].0 + mov.0, knots[0].1 + mov.1);
            for j in 1..10 {
                let head = &knots[j-1];
                let tail = &knots[j];

                // Now we process where the tail goes
                let x_diff = head.0 - tail.0;
                let y_diff = head.1 - tail.1;

                // Up right
                if (x_diff > 1 && y_diff > 0) || (x_diff > 0 && y_diff > 1) {
                    knots[j] = (tail.0+1, tail.1+1);
                } else if (x_diff > 1 && y_diff < 0) || (x_diff > 0 && y_diff < -1) {
                    knots[j] = (tail.0+1, tail.1-1);
                } else if (x_diff < 0 && y_diff < -1) || (x_diff < -1 && y_diff < 0) {
                    knots[j] = (tail.0-1, tail.1-1);
                } else if (x_diff < 0 && y_diff > 1) || (x_diff < -1 && y_diff > 0) {
                    knots[j] = (tail.0-1, tail.1+1);
                } else if x_diff > 1 {
                    knots[j] = (tail.0+1, tail.1);
                } else if x_diff < -1 {
                    knots[j] = (tail.0-1, tail.1);
                } else if y_diff > 1 {
                    knots[j] = (tail.0, tail.1+1);
                } else if y_diff < -1 {
                    knots[j] = (tail.0, tail.1-1);
                }
            }
            ref_map.insert(knots[9]);
        }

    }  

    ref_map.len()
}

fn main() {
    let filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/rope/data/data.txt";
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read file");
    let lines: Vec<&str> = contents.split('\n').collect();

    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
