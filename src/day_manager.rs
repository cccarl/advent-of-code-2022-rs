mod day1;
mod day2;
mod day3;
mod day4;

pub fn execute_day(day: u32) {
    println!("Running day: {}", day);

    match day {
        1 => day1::day_1_main(),
        2 => day2::day_2_main(),
        3 => day3::day_3_main(),
        4 => day4::day_4_main(),
        _ => {
            println!(
                "This day doesn't exist or doesn't have an exercise assigned! Ending program..."
            )
        }
    }
}
