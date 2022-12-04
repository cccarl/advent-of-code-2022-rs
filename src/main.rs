use std::env;
use std::fs;
use std::io::Write;
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

    // create template files for each day
    for i in 5..26 {
        let mut file = fs::File::create(format!("src/day_manager/day{}.rs", i)).unwrap();

        let contents_iter = format!("use std::fs;\n\n\
            pub fn day_{}_main() {{\n    \
                // https://adventofcode.com/2022/day/{}\n    \
                let file_path = \"inputs/placeholder.txt\";\n    \
                let input = fs::read_to_string(file_path).expect(\"Could not read or find file.\");\n    \
                todo!();\n\
            }}\n\
            ", i, i);

        let Ok(_) = file.write(contents_iter.as_bytes()) else {
            println!("Could not write to file.");
            return;
        };
    }

}


