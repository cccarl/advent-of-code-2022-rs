use std::fs;

pub fn day_5_main() {
    // https://adventofcode.com/2022/day/5
    let file_path = "inputs/placeholder.txt";
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    println!("\n{}\n", input);
    todo!();
}
