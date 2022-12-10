use std::{env, process::exit};
mod day_manager;

fn main() {
    // save the first command (the exercise to execute) or use a default
    let default_day = 1;
    let ex: u32 = env::args()
                        .nth(1)
                        .unwrap_or_else(|| {println!("\nCould not find console command, using {}.\nEnter a number to execute another exercise.", default_day); default_day.to_string()})
                        .parse()
                        .unwrap_or_else(|_| {println!("\nCould not parse console command into a number, aborting........"); exit(0);});

    let input_file = env::args().nth(2);

    // run the chosen day exercise
    day_manager::execute_day(ex, input_file.clone());
}
