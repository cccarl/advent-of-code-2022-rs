mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn execute_day(day: u32) {
    println!("\n***Running day: {}***\n", day);

    match day {
        1 => day1::day_1_main(),
        2 => day2::day_2_main(),
        3 => day3::day_3_main(),
        4 => day4::day_4_main(),
        5 => day5::day_5_main(),
        6 => day6::day_6_main(),
        7 => day7::day_7_main(),
        8 => day8::day_8_main(),
        9 => day9::day_9_main(),
        10 => day10::day_10_main(),
        11 => day11::day_11_main(),
        12 => day12::day_12_main(),
        13 => day13::day_13_main(),
        14 => day14::day_14_main(),
        15 => day15::day_15_main(),
        16 => day16::day_16_main(),
        17 => day17::day_17_main(),
        18 => day18::day_18_main(),
        19 => day19::day_19_main(),
        20 => day20::day_20_main(),
        21 => day21::day_21_main(),
        22 => day22::day_22_main(),
        23 => day23::day_23_main(),
        24 => day24::day_24_main(),
        25 => day25::day_25_main(),
        _ => {
            println!(
                "This day doesn't exist or doesn't have an exercise assigned! Ending program..."
            )
        }
    }
}
