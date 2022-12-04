use std::fs;

pub fn day_1_main() {
    // https://adventofcode.com/2022/day/1
    let file_path = "inputs/day_1_calories.txt";
    println!("File to open: {}", file_path);   

    // read file
    let calories_input = fs::read_to_string(file_path).expect("Could not read or find file.");

    //println!("Contents: \n{}", calories_input);

    // get all the sums in a vector
    let calory_sums: Vec<u32> = parse_calories(calories_input);

    println!("The entire vector of sums is: \n{:?}\n", calory_sums);

    let top_3_sum = get_top_3_sum(calory_sums.clone());
    println!("The sum of the top 3 times is: {}", top_3_sum);

}


fn parse_calories(input: String) -> Vec<u32> {

    let mut sums: Vec<u32> = vec![];

    let mut accum: u32 = 0;
    for line in  input.lines() {

        let parsed_int = match line.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                sums.push(accum);
                accum = 0;
                continue;
            }
        };

        accum += parsed_int;
    }

    sums
}

fn get_top_3_sum (mut input: Vec<u32>) -> u32 {

    input.sort();

    let mut final_sum = 0;
    let mut i = 1;
    while i <= 3 {

        final_sum += input[input.len() - i];

        i += 1;
    }

    final_sum
}

