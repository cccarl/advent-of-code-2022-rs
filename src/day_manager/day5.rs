use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct Instruction {
    quantity: i32,
    from: i32,
    to: i32,
}

pub fn day_5_main() {
    // https://adventofcode.com/2022/day/5
    let file_path = "inputs/day_5_cranes.txt";
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    //println!("\n{}\n", input);

    // 1st: put the stacks into vecs, the width is consistent so...
    let stacks_num: usize = (input.lines().next().unwrap().len() + 1) / 4;
    println!("NUMBER OF STACKS: {}", stacks_num);

    // the stacks are saved in a vector
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); stacks_num];

    // go stack by stack saving the contents of them
    for curr_stack in 0..stacks_num {
        for line in input.lines() {
            // jump to next stack once the stack is done, indicated by an empty line
            if line.is_empty() {
                break;
            }

            let mut line_iter = line.chars();

            // skip first char, and ignore line with stack ids at the bottom of them
            if line_iter.next().unwrap() == ' ' {
                continue;
            }

            // jump ahead into the actual stack data
            for _ in 0..(curr_stack * 4) {
                line_iter.next();
            }

            // save data in vec
            match line_iter.next() {
                Some(c) => {
                    // only save when it isn't an space
                    match c {
                        ' ' => continue,
                        _ => {
                            stacks[curr_stack].push_front(c);
                        }
                    }
                }
                None => println!("Tried to add data to a stack from non existing text. How did this happen?"),
            }
        }
    }

    println!("Stacks parsed, result:");
    for stack in &stacks {
        println!("{:?}", stack);
    }

    // process instructions
    for line in input.lines() {
        // ignore lines that do not contain instruction keywords
        if !line.contains("move") || !line.contains("from") || !line.contains("to") {
            continue;
        }

        // split sring by space, then filter every string slice that's not a number, the pattern is always the same, so...
        let instruction_parsed: Vec<i32> = line
            .split(' ')
            .filter(|x| x.chars().all(|y| y.is_numeric()))
            .map(|z| z.parse().unwrap())
            .collect();

        let instruction = Instruction {
            quantity: instruction_parsed[0],
            from: instruction_parsed[1],
            to: instruction_parsed[2],
        };
        println!("INSTRUCTION PARSED: {:?}", instruction);

        /* for part 1
        // move crate x number of times
        for _ in 0..instruction.quantity {
            let crate_item =  stacks[instruction.from as usize - 1 as usize].pop_back();

            match crate_item {
                Some(num) => stacks[instruction.to as usize - 1 as usize].push_back(num),
                None => println!("Instructions are wrong, tried to move more crates in an empty stack.")
            }
        }
        */

        // for part 2
        // we make an aux vector to push the contents twice and keep the correct order
        let mut auxvec: VecDeque<char> = VecDeque::new();
        for _ in 0..instruction.quantity {
            let crate_item = stacks[instruction.from as usize - 1 as usize].pop_back();

            match crate_item {
                Some(num) => auxvec.push_front(num),
                None => {
                    println!("Instructions are wrong, tried to move crates in an empty stack.")
                }
            }
        }

        for item in auxvec {
            stacks[instruction.to as usize - 1 as usize].push_back(item);
        }

        for stack in &stacks {
            println!("{:?}", stack);
        }
    }
}
