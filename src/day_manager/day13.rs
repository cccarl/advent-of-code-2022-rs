use std::fs;

#[derive(Debug, Clone)]
enum SignalElement {
    Number(u32),
    List(Vec<SignalElement>),
}

pub fn day_13_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/13
    let file_path = match input_file {
        Some(file) => file,
        None => "inputs/day_13_distress_signal_example.txt".to_owned(),
    };
    let input = fs::read_to_string(&file_path).expect(&format!(
        "\n\nCould not read or find file: {}\n\n",
        file_path.clone()
    ));
    println!("\n{}\n", input);
    
    println!("-------------------------Test start---------------");

    let test_str = "[[1],[2,3,4]]";
    let test_str2 = "[[4,4],4,4,4]";

    println!("Parsing:\n{}\n{}\n", test_str, test_str2);
    println!("\nstr 1 parsed: {:?}", parse_signal_side(test_str.strip_prefix("[").unwrap().strip_suffix("]").unwrap()));
    println!("\nstr 2 parsed: {:?}", parse_signal_side(test_str2.strip_prefix("[").unwrap().strip_suffix("]").unwrap()));

    println!("-------------------------Test end----------------");
/* 
    let mut signals: Vec<Vec<SignalElement>> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        signals.push(parse_signal_side(line.strip_prefix("[").unwrap().strip_suffix("]").unwrap()));
    }

    for signal in signals {
        println!("{:?}", signal);
    }*/
}

// gets a line with a signal side and return a vec with each element (a num or another list)
fn parse_signal_side(signal: &str) -> Vec<SignalElement> {

    let mut items: Vec<SignalElement> = vec![];
    let mut bracket_count = 0;

    for (pos, char) in signal.chars().enumerate() {

        match char {
            '[' => {
                if bracket_count == 0 {
                    items.push(deconstruct_signal_list(&signal[pos..], &mut vec![], 0));
                }
                bracket_count += 1;
            },
            ']' => {
                bracket_count -= 1;
            },
            ',' => {
                continue;
            }
            _ => {
                if bracket_count == 0 {
                    items.push(SignalElement::Number(char.to_string().parse().expect(&format!("Could not parase number in input: {}", char))));
                }
            }
        }
    }

    items
    
}

// i can't believe this worked
// gets a list of signal types in a string and recursively builds it into the proper SignalElement::List(_) enum using recursion
// TODO: change return to Option<SignalElement> to be able to return None and avoid the double nested lists
fn deconstruct_signal_list(partial_signal: &str, final_vec: &mut Vec<SignalElement>, bracket_count: usize) -> SignalElement {


    println!("Vec: {:?} -- Current slice: {} -- Bracket count: {}", final_vec, partial_signal, bracket_count);
    match partial_signal.chars().next() {
        Some(c) => {
            match c {
                '[' => {
                    final_vec.push(deconstruct_signal_list(&partial_signal[1..], &mut vec![], bracket_count + 1));
                    return SignalElement::List(final_vec.to_vec());
                },
                ']' => {
                    if bracket_count != 1 {
                        deconstruct_signal_list(&partial_signal[1..], final_vec, bracket_count - 1);
                        return SignalElement::List(final_vec.to_vec());
                    }
                    else {
                        // this should not happen question mark?
                        return SignalElement::Number(10000);
                    }
                    
                },
                ',' => {
                    deconstruct_signal_list(&partial_signal[1..], final_vec, bracket_count);
                    return SignalElement::List(final_vec.to_vec());
                },
                _ => {
                    final_vec.push(SignalElement::Number(c.to_string().parse().expect(&format!("Could not parase number in input: {}", c))));
                    deconstruct_signal_list(&partial_signal[1..], final_vec, bracket_count);
                    return SignalElement::List(final_vec.to_vec());
                }
            }

        },
        None => return SignalElement::Number(420),
    }

}