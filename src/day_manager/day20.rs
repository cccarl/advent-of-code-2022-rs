use std::fs;

pub fn day_20_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/20
    let file_path = match input_file {
        Some(file) => file,
        None => "inputs/placeholder.txt".to_owned(),
    };
    let input = fs::read_to_string(&file_path).expect(&format!(
        "\n\nCould not read or find file: {}\n\n",
        file_path.clone()
    ));
    println!("\n{}\n", input);
    todo!();
}
