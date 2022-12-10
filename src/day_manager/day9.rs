use core::time;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Rope {
    knots_x: Vec<i32>,
    knots_y: Vec<i32>,
}

#[derive(Debug)]
enum RopeMovement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    movement: RopeMovement,
    quantity: u32,
}

impl Rope {
    // move head to a direction, them move every other knot of the rope accordingly
    fn move_head(&mut self, mov: &RopeMovement, ammount: u32) {
        for _ in 0..ammount {
            match mov {
                RopeMovement::Up => {
                    // move head knot
                    self.knots_y[0] += 1;

                    // iterate over the knots to move them if they are too spearated
                    for i in 0..self.knots_x.len() - 1 {
                        if !self.move_tail(i) {
                            break;
                        }
                    }
                }
                RopeMovement::Down => {
                    self.knots_y[0] -= 1;

                    for i in 0..self.knots_x.len() - 1 {
                        if !self.move_tail(i) {
                            break;
                        }
                    }
                }
                RopeMovement::Left => {
                    self.knots_x[0] -= 1;

                    for i in 0..self.knots_x.len() - 1 {
                        if !self.move_tail(i) {
                            break;
                        }
                    }
                }
                RopeMovement::Right => {
                    self.knots_x[0] += 1;

                    for i in 0..self.knots_x.len() - 1 {
                        if !self.move_tail(i) {
                            break;
                        }
                    }
                }
            }
        }
    }

    // called by move_head to properly move the tail after a head movement, calling it manually should have no effect
    // return a bool indicating if there was movement or not
    fn move_tail(&mut self, i: usize) -> bool {
        let mut movement = true;
        let (dist_x, dist_y) = calc_distance(
            (self.knots_x[i], self.knots_y[i]),
            (self.knots_x[i + 1], self.knots_y[i + 1]),
        );
        // cover every possible movement in one big function, part 2 added the case of both distances being 2 (it was a pain to find that out)
        if (dist_y >= 2 && dist_x == -1)
            || (dist_x == -2 && dist_y == 2)
            || (dist_x <= -2 && dist_y == 1)
        {
            self.knots_x[i + 1] -= 1;
            self.knots_y[i + 1] += 1;
        } else if (dist_y <= -2 && dist_x == 1)
            || (dist_x >= 2 && dist_y == -1)
            || (dist_x == 2 && dist_y == -2)
        {
            self.knots_x[i + 1] += 1;
            self.knots_y[i + 1] -= 1;
        } else if (dist_y >= 2 && dist_x == 1)
            || (dist_x >= 2 && dist_y == 1)
            || (dist_x == 2 && dist_y == 2)
        {
            self.knots_x[i + 1] += 1;
            self.knots_y[i + 1] += 1;
        } else if (dist_y <= -2 && dist_x == -1)
            || (dist_x <= -2 && dist_y == -1)
            || (dist_x == -2 && dist_y == -2)
        {
            self.knots_x[i + 1] -= 1;
            self.knots_y[i + 1] -= 1;
        } else if dist_y >= 2 && dist_x == 0 {
            self.knots_y[i + 1] += 1;
        } else if dist_y <= -2 && dist_x == 0 {
            self.knots_y[i + 1] -= 1;
        } else if dist_x <= -2 && dist_y == 0 {
            self.knots_x[i + 1] -= 1;
        } else if dist_x >= 2 && dist_y == 0 {
            self.knots_x[i + 1] += 1;
        } else {
            movement = false;
        }

        movement
    }
}

pub fn day_9_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/9
    let file_path = match input_file {
        None => "inputs/day_9_rope.txt".to_owned(),
        Some(file) => file,
    };
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    println!("\n{}\n", input);

    // so we now have a rope with a head and tail, the tail will follow the head as long as there are 2 spaces separating them. in a grid.
    // the square surrounding the head counts as 1 space, the tail will move 1 space to keep up, if that's not enough, it moves diagonally

    // part 2 edit: refaftor rope into being 2 vecs of x and y to make rope length arbitrary
    let mut test_rope = Rope {
        knots_x: vec![0, -1, -2],
        knots_y: vec![0, -1, -1],
    };

    println!("Test Rope: {:?}", test_rope);
    test_rope.move_head(&RopeMovement::Right, 1);
    println!("Test rope after moving 1 up: {:?}", test_rope);

    let mut hash_set_test = HashSet::new();
    hash_set_test.insert((1, 2));
    hash_set_test.insert((0, 0));
    hash_set_test.insert((0, 0));
    println!("Test hash set: {:?}", hash_set_test);

    // ok struct tests done, time for the real thing
    // parse input
    let mut instructions: Vec<Instruction> = vec![];
    for ins_str in input.lines() {
        let mut ins_chars = ins_str.chars();
        let movement = match ins_chars.next() {
            Some(c) => match c {
                'U' => Some(RopeMovement::Up),
                'D' => Some(RopeMovement::Down),
                'L' => Some(RopeMovement::Left),
                'R' => Some(RopeMovement::Right),
                _ => {
                    println!("Could not recognize character in line: {}", ins_str);
                    None
                }
            },
            None => {
                println!("This line is empty somehow.");
                None
            }
        };

        ins_chars.next();
        let quantity = ins_chars
            .collect::<String>()
            .parse()
            .expect(&format!("The 3rd+ char should be a number: {}", ins_str));
        instructions.push(Instruction {
            movement: movement.unwrap(),
            quantity: quantity,
        });
    }

    println!("{:?}", instructions);

    // PART 1: count how many spaces goes the TAIL through
    // PART 2: same as before, BUT now the rope has 10 knots, this means that you move the head then a chain reaction occurs until the end of the rope
    // refactoring the Rope struct and changing the move_head func to apply to every knot solves part 2 and any arbitrary length
    let mut tail_trail = HashSet::new();
    let rope_len = 10; // part1: 2, part2: 10
    let mut rope = Rope {
        knots_x: vec![0; rope_len],
        knots_y: vec![0; rope_len],
    };
    for inst in instructions {
        for _ in 0..inst.quantity {
            // yea don't want to refactor the ammount param, without this 'for' instead of just using the param i can't save the entire trail easily
            rope.move_head(&inst.movement, 1);
            tail_trail.insert((rope.knots_x[rope_len - 1], rope.knots_y[rope_len - 1]));
        }
    }
    println!("The final hashset is: {:?}", tail_trail);
    println!("The ammount of trails is: {}", tail_trail.len());
}

// get the distance between the head and tail, separated in x and y
fn calc_distance(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dist_x = head.0 - tail.0;
    let dist_y = head.1 - tail.1;
    if dist_x > 2 || dist_x < -2 || dist_y > 2 || dist_y < -2 {
        println!("WARNING: This dist should not be possible: {dist_x} {dist_y}");
        std::thread::sleep(time::Duration::from_millis(10000));
    }
    return ((dist_x), (dist_y));
}
