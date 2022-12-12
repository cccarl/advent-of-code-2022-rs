use std::{fs};


#[derive(Default, Debug, Clone)]
enum Operation {
    Sum(i32),
    Multiply(i32),
    #[default]
    SumSelf,
    MultiplySelf
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    items: Vec<i128>,
    op: Operation,
    test_divisor: i32,
    test_true: usize,
    test_false: usize,
}

pub fn day_11_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/11
    let file_path = match input_file {
        Some(file) => file,
        None => "inputs/day_11_monkey_business_example.txt".to_owned(),
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
        let monkey_split: Vec<&str> = monkey_str.split(&['\n']).filter(|x| !x.is_empty()).collect();
        if monkey_split.len() == 0 {
            continue;
        }
        let mut monkey: Monkey = Default::default();
        for feature in monkey_split {
            // save the items
            if feature.contains("Starting items:") {
                let items: Vec<&str> = feature.split("  Starting items: ").filter(|x| x != &"").collect();
                monkey.items = items[0].split(", ").map(|n| n.parse().unwrap()).collect();
                
            }
            // save the operation
            else if feature.contains("Operation:") {
                let operation_vec: Vec<&str> = feature.split("Operation: new = old ").collect::<Vec<&str>>()[1].split(' ').collect::<Vec<&str>>();

                let op;
                let op_value;
                match (operation_vec[0], operation_vec[1]) {
                    ("+", "old") => {
                        op = Operation::SumSelf
                    },
                    ("*", "old") => {
                        op = Operation::MultiplySelf
                    },
                    ("+", _) => {
                        op_value = operation_vec[1].parse().expect("Could not pase operation number");
                        op = Operation::Sum(op_value);
                    },
                    ("*", _) => {
                        op_value = operation_vec[1].parse().expect("Could not pase operation number");
                        op = Operation::Multiply(op_value);
                    },
                    (_, _) => {
                        op = Default::default();
                        println!("Could not find a known pattern for the operation, using the default: {:?}", op);
                    }
                }
                monkey.op = op;

            }
            // test
            else if feature.contains("Test:") {
                monkey.test_divisor = feature.strip_prefix("  Test: divisible by ").unwrap().parse().unwrap();
            }
            // test when true
            else if feature.contains("If true:") {
                monkey.test_true = feature.strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap();
            }
            // test when false
            else if feature.contains("If false:") {
                monkey.test_false = feature.strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap();
            }
            
        }
        monkeys.push(monkey);
    }

    println!("Monkeys parsed!");
    for monkey in &monkeys {
        println!("{monkey:?}");
    }

    // start the monkey madness
    let mut inspection_counts = vec![0; monkeys.len()];
    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // inspect each element and throw it
            for (j, item_worry) in monkeys[i].clone().items.iter().enumerate() {
                //println!("Testing j item: {j} {item_worry} ");
                // apply monkey operation
                let mut new_worry = match monkeys[i].op {
                    Operation::Sum(n) => item_worry + i128::from(n),
                    Operation::Multiply(n) => {
                        let mut new_worry = item_worry * i128::from(n);
                        //new_worry = new_worry % i128::from(monkeys[i].test_divisor);
                        new_worry
                    },
                    Operation::SumSelf => item_worry + item_worry,
                    Operation::MultiplySelf => {
                        let mut new_worry = item_worry * item_worry;
                        //new_worry = new_worry % i128::from(monkeys[i].test_divisor);
                        new_worry

                    },
                };
                

                // divide worry level by 3
                // PART 2: remove this relief
                new_worry /= 3;
                //replacement for part 2, this makes it possible to still have an accurate operation
                
                // idk

                
                // apply monkey test
                if new_worry % i128::from(monkeys[i].test_divisor) == 0{
                    let receiver_index = monkeys[i].test_true;
                    monkeys[receiver_index].items.push(new_worry);
                    monkeys[i].items.remove(0);
                } else {
                    let receiver_index = monkeys[i].test_false;
                    monkeys[receiver_index].items.push(new_worry);
                    monkeys[i].items.remove(0);
                }

                inspection_counts[i] += 1;
/*
                for monkey in &monkeys {
                    println!("New Monkeys -- {:?}", monkey);
                }
*/              
            }
        }
    }

    // part 1 answer
    println!("Inspections end: {:?}", inspection_counts);
    let mut sorted_inspections = inspection_counts.clone();
    // part 2 can't handle this sort lol
    sorted_inspections.sort();
    let monkey_business = sorted_inspections[sorted_inspections.len()-1] * sorted_inspections[sorted_inspections.len()-2];
    println!("MONKEY BUSINESS: {}", monkey_business);




}
