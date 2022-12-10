mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
//mod day7;
mod day8;
mod day9;

pub fn execute_day(day: u32, input_file: Option<String>) {
    println!("\n***Running day: {}***\n", day);

    match day {
        1 => day1::day_1_main(input_file),
        2 => day2::day_2_main(input_file),
        3 => day3::day_3_main(input_file),
        4 => day4::day_4_main(input_file),
        5 => day5::day_5_main(input_file),
        6 => day6::day_6_main(input_file),
        //7 => day7::day_7_main(input_file),
        8 => day8::day_8_main(input_file),
        9 => day9::day_9_main(input_file),
        10 => day10::day_10_main(input_file),
        11 => day11::day_11_main(input_file),
        12 => day12::day_12_main(input_file),
        13 => day13::day_13_main(input_file),
        14 => day14::day_14_main(input_file),
        15 => day15::day_15_main(input_file),
        16 => day16::day_16_main(input_file),
        17 => day17::day_17_main(input_file),
        18 => day18::day_18_main(input_file),
        19 => day19::day_19_main(input_file),
        20 => day20::day_20_main(input_file),
        21 => day21::day_21_main(input_file),
        22 => day22::day_22_main(input_file),
        23 => day23::day_23_main(input_file),
        24 => day24::day_24_main(input_file),
        25 => day25::day_25_main(input_file),
        _ => {
            println!(
                "This day doesn't exist or doesn't have an exercise assigned! Ending program..."
            )
        }
    }
}
