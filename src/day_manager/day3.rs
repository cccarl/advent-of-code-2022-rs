use std::collections::HashMap;
use std::fs;

pub fn day_3_main() {
    // https://adventofcode.com/2022/day/3

    /*
    -runsack with items in 2 compartments
    -each compartment has certain types of items
    -lol oops an elf made woppsie and mixed up certain item type, THERE IS ALWAYS 1 ERROR PER SACK

    -INPUT: file with letters representing diff item TYPE
    -each line is a runsack
    -each half of the lines is each compartment of the runsack, they are perfectly split
    -each letter has a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

    PART 1: find sum of priorities of the mistaken types of each sack.

    PART 2: find the medal id for groups of 3 elves (3 lines in a row)
    and add their priorities, shold be the only repeat items from
    those 3 lines
    */

    let file_path = "inputs/day_3_rucksacks.txt";
    let sacks_raw = fs::read_to_string(file_path).expect("Could not read or find file");

    run_part_1(sacks_raw);

    // lazy fix to satisfy the borrow checker for the 2nd func
    let file_path = "inputs/day_3_rucksacks.txt";
    let sacks_raw = fs::read_to_string(file_path).expect("Could not read or find file");
    run_part_2(sacks_raw);

}

fn get_priority_hashmap() -> HashMap<char, usize> {
    // generate vec of chars with abc lower then uppercase
    let mut alphabet = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();

    alphabet.extend((b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>());

    let mut priorities: HashMap<char, usize> = HashMap::new();

    for letter in &alphabet {
        priorities.insert(
            *letter,
            alphabet.iter().position(|&x| x == *letter).unwrap() + 1,
        );
        println!(
            "Added enty to hashmap: {} -> {}",
            letter, priorities[&letter]
        );
    }

    println!("The priority of a is: {}", priorities[&'b']);

    priorities
}

fn run_part_1(input: String) {
    // create a hashmap with the priorities for convenience
    let priorities_map = get_priority_hashmap();

    // check which letter is repeated, then add to sum
    let mut priority_sum = 0;
    for ruck in input.lines() {
        let mut comp1: Vec<char> = vec![];
        let mut comp2: Vec<char> = vec![];

        for item in ruck.chars() {
            if comp1.len() < ruck.len() / 2 {
                comp1.push(item.clone());
            } else {
                comp2.push(item.clone());
            }
        }
        println!("\nComp1: {:?}\nComp2: {:?}", comp1, comp2);

        for item in &comp1 {
            if comp2.contains(item) {
                priority_sum += priorities_map[item];
                println!(
                    "Found the repeat item: {}, adding {} to the priority sum.",
                    *item, priorities_map[item]
                );
                // WE KNOW there is always only 1 repeat
                break;
            }
        }
    }

    println!("\nAnd the final sum is: {}", priority_sum);
}

fn run_part_2(input: String) {
    // create a hashmap with the priorities for convenience
    let priorities_map = get_priority_hashmap();

    // find the repeat letter among 3 lines
    let mut priority_sum = 0;
    let mut member_num = 0; // repeat in a cycle of 0 1 2
    let mut elf_group: [Vec<char>; 3] = [vec![], vec![], vec![]];
    for ruck in input.lines() {
        // save the group data
        for letter in ruck.chars() {
            elf_group[member_num].push(letter);
        }

        // once a group is complete, check the repeated letter
        if member_num == 2 {
            println!(
                "Group complete!\n1: {:?}\n2: {:?}\n3: {:?}\n",
                elf_group[0], elf_group[1], elf_group[2]
            );

            for item in &elf_group[0] {
                if elf_group[1].contains(&item) && elf_group[2].contains(&item) {
                    println!("Found medal: {}\n", item);
                    priority_sum += priorities_map[&item];
                    break;
                }
            }

            let mut i = 0;
            while i < elf_group.len() {
                elf_group[i].clear();
                i += 1;
            }

            member_num = 0
        } else {
            member_num += 1
        }
    }

    println!("Final sum is: {}", priority_sum);
}
