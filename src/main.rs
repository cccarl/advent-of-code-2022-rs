use std::env;
use std::fs;
mod day_manager;

fn main() {
    // save console variables
    let mut console_args = env::args();

    // save the first command (the exercise to execute) or use a default
    let default = 1;
    let ex: u32 = console_args
                        .nth(1)
                        .unwrap_or_else(|| {println!("\nCould not find console command, using {}.\nEnter a number to execute another exercise.", default); default.to_string()})
                        .parse()
                        .unwrap_or_else(|_| {println!("\nCould not parse console command into number, using {}.", default); default});

    day_manager::execute_day(ex);


}
