use itertools::Itertools;
use std::{collections::VecDeque, fs};

pub fn day_6_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/6
    let file_path = match input_file {
        None => "inputs/day_6_data_stream.txt".to_owned(),
        Some(file) => file,
    };
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");

    // PART 1
    unique_chars_pos(input.clone(), 4);

    // PART 2
    unique_chars_pos(input.clone(), 14);
}

/**
 * get the position of the point of the input where there are only unique characters
 */
fn unique_chars_pos(input: String, unique_len: u32) -> u32 {
    let mut input_as_chars = input.chars();
    let mut possible_start: VecDeque<char> = VecDeque::new();
    // build the first start candidate unconditionally
    for _ in 0..unique_len {
        possible_start.push_front(input_as_chars.next().unwrap());
    }

    // check if it's the start, if it isn't move on to the next character
    let mut start_pos = unique_len;
    println!(
        "Number of unique characters to search: {}.\nStarting search...",
        unique_len
    );
    for char in input_as_chars {
        // check if filtering the unique chars in the possible start has a len of the code, if it does we found the signal start
        if possible_start
            .clone()
            .into_iter()
            .unique()
            .collect::<Vec<char>>()
            .len()
            == unique_len.try_into().unwrap()
        {
            println!(
                "Found start! Start position: {}\n{:?}",
                start_pos, possible_start
            );
            break;
        }

        // move on to the next character
        // note that the vecdequeue makes it possible to keep adding to the front of the pattern and removing to its back
        possible_start.pop_back();
        possible_start.push_front(char);
        start_pos += 1;
    }

    start_pos
}
