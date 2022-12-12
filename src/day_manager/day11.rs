use std::fs;

use itertools::Itertools;

#[derive(Default, Debug, Clone)]
enum Operation {
    Sum(i32),
    Multiply(i32),
    #[default]
    SumSelf,
    MultiplySelf,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    op: Operation,
    test_divisor: i64,
    test_true: usize,
    test_false: usize,
}

pub fn day_11_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/11
    let file_path = match input_file {
        Some(file) => file,
        None => "inputs/day_11_monkey_business.txt".to_owned(),
    };
    let input = fs::read_to_string(&file_path).expect(&format!(
        "\n\nCould not read or find file: {}\n\n",
        file_path.clone()
    ));
    //println!("\n{}\n", input);

    // not worth explaining here, TOO MANY DAMN monkeys
    // q: what is the monkey business value after 20 rounds?
    // that value is the multiplication of the number of inspected items of the top 2 most active monkeys (the ones that inspected the most times)

    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_str in input.split("Monkey").collect::<Vec<&str>>() {
        let monkey_split: Vec<&str> = monkey_str
            .split(&['\n'])
            .filter(|x| !x.is_empty())
            .collect();
        if monkey_split.len() == 0 {
            continue;
        }
        let mut monkey: Monkey = Default::default();
        for feature in monkey_split {
            // save the items
            if feature.contains("Starting items:") {
                let items: Vec<&str> = feature
                    .split("  Starting items: ")
                    .filter(|x| x != &"")
                    .collect();
                monkey.items = items[0].split(", ").map(|n| n.parse().unwrap()).collect();
            }
            // save the operation
            else if feature.contains("Operation:") {
                let operation_vec: Vec<&str> = feature
                    .split("Operation: new = old ")
                    .collect::<Vec<&str>>()[1]
                    .split(' ')
                    .collect::<Vec<&str>>();

                let op;
                let op_value;
                match (operation_vec[0], operation_vec[1]) {
                    ("+", "old") => op = Operation::SumSelf,
                    ("*", "old") => op = Operation::MultiplySelf,
                    ("+", _) => {
                        op_value = operation_vec[1]
                            .parse()
                            .expect("Could not pase operation number");
                        op = Operation::Sum(op_value);
                    }
                    ("*", _) => {
                        op_value = operation_vec[1]
                            .parse()
                            .expect("Could not pase operation number");
                        op = Operation::Multiply(op_value);
                    }
                    (_, _) => {
                        op = Default::default();
                        println!("Could not find a known pattern for the operation, using the default: {:?}", op);
                    }
                }
                monkey.op = op;
            }
            // test
            else if feature.contains("Test:") {
                monkey.test_divisor = feature
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            // test when true
            else if feature.contains("If true:") {
                monkey.test_true = feature
                    .strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            // test when false
            else if feature.contains("If false:") {
                monkey.test_false = feature
                    .strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
            }
        }
        monkeys.push(monkey);
    }

    println!("Monkeys parsed!");
    for monkey in &monkeys {
        println!("{monkey:?}");
    }

    // part 1 answer: 20 rounds with a relief of 3
    println!("Part 1 ans: {}\n", do_monkey_rounds(monkeys.clone(), 20, 3));

    // part 2 answer: 10000 rounds with no relief (1)
    println!(
        "Part 2 ans: {}\n",
        do_monkey_rounds(monkeys.clone(), 10000, 1)
    );
}

fn do_monkey_rounds(mut monkeys: Vec<Monkey>, rounds: u32, relief: i64) -> u64 {
    // start the monkey madness
    let mut inspection_counts = vec![0; monkeys.len()];
    println!("Starting the monkey rounds... Params: rounds {rounds}, refief {relief}");

    // for part 2: math (this is to reduce the items to managable numbers)
    let divisors_product = monkeys.iter().map(|m| m.test_divisor).product::<i64>();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // inspect each element and throw it
            for (_j, item_worry) in monkeys[i].clone().items.iter().enumerate() {
                //println!("Testing j item: {j} {item_worry} ");
                // apply monkey operation
                let mut new_worry: i64 = match monkeys[i].op {
                    Operation::Sum(n) => i64::from(item_worry + n),
                    Operation::Multiply(n) => i64::from(item_worry * n),
                    Operation::SumSelf => i64::from(item_worry + item_worry),
                    Operation::MultiplySelf => i64::from(*item_worry) * i64::from(*item_worry),
                };

                // divide worry level by 3 (part 2: relief is 0, let's make it a param)
                new_worry /= relief;
                // for part 2: make the new worry small enough to be managable and keep the divisibility
                new_worry %= divisors_product;

                // apply monkey test
                if new_worry % monkeys[i].test_divisor == 0 {
                    let receiver_index = monkeys[i].test_true;
                    monkeys[receiver_index]
                        .items
                        .push(new_worry.try_into().unwrap());
                    monkeys[i].items.remove(0);
                } else {
                    let receiver_index = monkeys[i].test_false;
                    monkeys[receiver_index]
                        .items
                        .push(new_worry.try_into().unwrap());
                    monkeys[i].items.remove(0);
                }

                inspection_counts[i] += 1;
            }
        }
    }

    println!("Finish! Inspections count: {:?}", inspection_counts);
    // don't judge me for this mess, lazy mode of sort() crashes because of the big numbers
    // all i wanted was to collect the 2 highest numebers of the vector
    let highest_insps_vec = inspection_counts.iter().max_set();
    let highest_insp = *highest_insps_vec[0];

    if highest_insps_vec.len() == 1 {
        inspection_counts.retain(|&n| n != highest_insp);
        let second_highest = inspection_counts.iter().max().unwrap();
        return second_highest * highest_insp;
    } else {
        return u64::from(*highest_insps_vec[0]) * u64::from(*highest_insps_vec[1]);
    }
}
