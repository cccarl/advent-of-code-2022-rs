use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

pub fn day_10_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/10
    let file_path = match input_file {
        None => "inputs/day_10_cpu.txt".to_owned(),
        Some(file) => file,
    };
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    //println!("\n{}\n", input);

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
    let mut reg_x: i64 = 1; // starts on 1
    let mut cycle: i64 = 0;
    let mut signal_sum: i64 = 0;
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
                reg_x += i64::from(*c);
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
    let mut vblank = 0; // 16 cycles of stalling when a frame is drawn
    for instruction in &program {
        // bad apple compatibility, there are 16 cycles of nothing after a frame is drawn
        // shoutouts https://gist.github.com/RocketRace/3dd92cfc4ddfa614bca2de1b317e4406
        if vblank == 16 {
            vblank = 0;
            cycle = 1;

            print!("{esc}c", esc = 27 as char);
            sleep(Duration::from_millis(10));
        }

        match instruction {
            Instruction::Addx(n) => {
                // cycle 1
                if cycle > 240 {
                    //println!("Stalling... cycle {}, stalls {}", cycle, vblank);
                    vblank += 1;
                } else {
                    print_pixel(cycle, reg_x);
                }
                cycle += 1;

                // cycle 2
                if cycle > 240 {
                    //println!("Stalling... cycle {}, stalls {}", cycle, vblank);
                    vblank += 1;
                } else {
                    print_pixel(cycle, reg_x);
                }
                reg_x += *n;
                cycle += 1;
            }
            Instruction::Noop => {
                if cycle > 240 {
                    //println!("Stalling... cycle {}, stalls {}", cycle, vblank);
                    vblank += 1;
                } else {
                    print_pixel(cycle, reg_x);
                }
                cycle += 1;
            }
        }
    }
}

fn calc_signal_strength(cycle: i64, reg_x: i64) -> i64 {
    return cycle * reg_x;
}

fn print_pixel(curr_pixel: i32, reg_x: i32) {
    // quick jank bug fix: because of the way i wrote the cycles, 40 % 0 -> 0 so the reg_x is all wrong on the last column
    if (curr_pixel % 40 == 0)
        && (curr_pixel % 40 + 40 == reg_x
            || curr_pixel % 40 + 40 == reg_x + 1
            || curr_pixel % 40 + 40 == reg_x + 2)
    {
        print!("██");
    } else if (curr_pixel % 40 == 0)
        && (curr_pixel % 40 + 40 != reg_x
            || curr_pixel % 40 + 40 != reg_x + 1
            || curr_pixel % 40 + 40 != reg_x + 2)
    {
        print!("  ");
    }
    // non jank part
    else if curr_pixel % 40 == reg_x
        || curr_pixel % 40 == reg_x + 1
        || curr_pixel % 40 == reg_x + 2
    {
        print!("██");
    } else {
        print!("  ");
    }
    if curr_pixel % 40 == 0 {
        println!("");
    }
}
