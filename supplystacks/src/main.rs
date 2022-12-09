use std::fs;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    assert!(args.len() == 2);
    let mut buckets: i32 = 0;
    let mut filepath: &str = "";
        
    if args[1] == "test" {
        buckets = 3;
        filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/supplystacks/data/test.txt";
    } else if args[1] == "data" {
        buckets = 9;
        filepath = "/Users/arjuntaneja/Desktop/Programming Tings/advent_of_code_2022/supplystacks/data/data.txt";
    } else {
        println!("Incorrect argument to binary")
    }

    let mut stack_vec: Vec<Vec<char>> = Vec::new();
    for _ in 0..buckets {
        stack_vec.push(Vec::new());
    }
    
    let contents = fs::read_to_string(filepath)
        .expect("Failed to read file contents");
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut idx = 0;
    loop {
        let line = lines[idx];
        if line.as_bytes()[1] as char == '1' {
            break;
        } 

        for i in 0..buckets {
            let ch = line.as_bytes()[(4*i+1) as usize] as char;
            if ch != ' ' {
                stack_vec[i as usize].push(ch);
            }
        }

        idx += 1
    }

    for v in stack_vec.iter_mut() {
        v.reverse();
    }


    idx += 2; // Now, we are finally on the line with commands
    
    for i in idx..lines.len() {
        if lines[i].is_empty() { break; }
        let mut numbers: Vec<i32> = vec![];

        for token in lines[i].split(' ') {
            if token.parse::<i32>().is_ok() {
                numbers.push(token.parse::<i32>().unwrap());
            }
        }

        let qty = numbers[0];
        let source: usize = (numbers[1]-1).try_into().unwrap();
        let dest: usize = (numbers[2]-1).try_into().unwrap();

        let mut aux: Vec<char> = vec![];
        for _ in 0..qty {
            let item = stack_vec[source].pop().unwrap();
            aux.push(item);
        }

        for _ in 0..qty {
            let item = aux.pop().unwrap();
            stack_vec[dest].push(item);
        }


    }
    
    let mut output: String = "".to_owned();

    for v in stack_vec.iter() {
        output.push(*v.last().unwrap());
    }

    println!("{}", output);

}
