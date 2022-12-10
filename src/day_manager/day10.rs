use std::fs;

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

pub fn day_10_main() {
    // https://adventofcode.com/2022/day/10
    let file_path = "inputs/day_10_cpu.txt";
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    println!("\n{}\n", input);

    // save the program
    let mut program: Vec<Instruction> = vec![];
    for line in input.lines() {
        let split_line: Vec<&str> = line.split(' ').collect();

        program.push(match split_line[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(
                split_line[1]
                    .parse::<i32>()
                    .expect("Could not parse number for addx instruction"),
            ),
            _ => todo!(),
        })
    }

    // noop is 1 cycle
    // addx is 2 cycles
    // addx changes register X
    // PART 1: calc sum of signal strengths in cycles 20, 60, 100, 140, 180, 220
    // signal strength: cycle number * value in register X
    let mut reg_x = 1; // starts on 1
    let mut cycle = 0;
    let mut signal_sum = 0;
    for instruction in &program {
        cycle += 1;
        if cycle % 40 == 20 {
            signal_sum += calc_signal_strength(cycle, reg_x);
        }
        match instruction {
            Instruction::Addx(c) => {
                cycle += 1;
                if cycle % 40 == 20 {
                    signal_sum += calc_signal_strength(cycle, reg_x);
                }
                reg_x += c;
            }
            Instruction::Noop => { /* lol like a real noop */ }
        }
    }
    // PART 1 ans
    println!("Sum strength sum is: {}", signal_sum);

    // PART 2: how do i explain this...
    // RACE THE BEAM, a tv will draw a # or a . every frame in a 40x6 screen
    // if a sprite is overlapping the draw position, it's a #, otherwise it's a .
    // the sprite post is determined by the register X, and it's 3 spaces wide
    // X starts in 1 to the line is ###..................................... or smth
    // restult is whatever word the image draws

    // time to print the image
    let mut reg_x = 1; // starts on 1
    let mut cycle = 1;
    println!("Racing the beam:");
    for instruction in &program {
        match instruction {
            Instruction::Addx(c) => {
                print_pixel(cycle, reg_x);
                cycle += 1;
                print_pixel(cycle, reg_x);
                cycle += 1;
                reg_x += c;
            }
            Instruction::Noop => {
                print_pixel(cycle, reg_x);
                cycle += 1;
            }
        }
    }
}

fn calc_signal_strength(cycle: i32, reg_x: i32) -> i32 {
    return cycle * reg_x;
}

fn print_pixel(curr_pixel: i32, reg_x: i32) {
    // quick jank bug fix: because of the way i wrote the cycles, 40 % 0 -> 0 so the reg_x is all wrong on the last column
    if (curr_pixel % 40 == 0)
        && (curr_pixel % 40 + 40 == reg_x
            || curr_pixel % 40 + 40 == reg_x + 1
            || curr_pixel % 40 + 40 == reg_x + 2)
    {
        print!("█ ");
    } else if (curr_pixel % 40 == 0)
        && (curr_pixel % 40 + 40 != reg_x
            || curr_pixel % 40 + 40 != reg_x + 1
            || curr_pixel % 40 + 40 != reg_x + 2)
    {
        print!(". ");
    }
    // non jank part
    else if curr_pixel % 40 == reg_x
        || curr_pixel % 40 == reg_x + 1
        || curr_pixel % 40 == reg_x + 2
    {
        print!("█ ");
    } else {
        print!(". ");
    }
    if curr_pixel % 40 == 0 {
        println!("");
    }
}
