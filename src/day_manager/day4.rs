use std::fs;

struct Cleanup {
    start: i32,
    finish: i32,
}

pub fn day_4_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/4
    let file_path = match input_file {
        None => "inputs/day_4_cleanup.txt".to_owned(),
        Some(file) => file,
    };
    let cleanup_raw = fs::read_to_string(file_path).expect("Could not read or find file");

    let mut full_overlap_count = 0;
    let mut small_overlap_count = 0;
    for line in cleanup_raw.lines() {
        let round_nums: Vec<&str> = line.split(&['-', ',']).collect();

        let elf1 = Cleanup {
            start: round_nums[0].parse().unwrap(),
            finish: round_nums[1].parse().unwrap(),
        };

        let elf2 = Cleanup {
            start: round_nums[2].parse().unwrap(),
            finish: round_nums[3].parse().unwrap(),
        };

        // part 1: in how many cases there is a full overlap?
        if elf1.start <= elf2.start && elf1.finish >= elf2.finish
            || elf1.start >= elf2.start && elf1.finish <= elf2.finish
        {
            println!(
                "Full found! elf1: {}-{}, elf2: {}-{}",
                elf1.start, elf1.finish, elf2.start, elf2.finish
            );
            full_overlap_count += 1;
        }

        // part 2: in how many cases do any of the rooms repeat at all?
        // 3 cases considered: start of elf 2 contained in elf1, end of elf2 contained in elf1, or elf 1 is entirely contained in elf 2
        if elf1.start <= elf2.start && elf1.finish >= elf2.start
            || elf1.start <= elf2.finish && elf1.finish >= elf2.finish
            || elf1.start >= elf2.start && elf1.finish <= elf2.finish
        {
            println!(
                "Small overlap found! elf1: {}-{}, elf2: {}-{}",
                elf1.start, elf1.finish, elf2.start, elf2.finish
            );
            small_overlap_count += 1;
        }
    }

    println!("The FULL overlap count is: {}", full_overlap_count);
    println!("The SMALL overlap count is: {}", small_overlap_count);
}
