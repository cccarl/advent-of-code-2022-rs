use core::time;
use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Rope {
    head_x: i32,
    head_y: i32,
    tail_x: i32,
    tail_y: i32,
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

    fn move_head(&mut self, mov: &RopeMovement, ammount: u32) {

        for _ in 0..ammount {
            match mov {
                RopeMovement::Up => {
                    self.head_y += 1;
                    let (dist_x, dist_y) = self.calc_distance();
                    // cover 3 possible positions where the tail should be moved
                    if dist_y >= 2 && dist_x == -1 {
                        self.tail_x -= 1;
                        self.tail_y += 1;
                    }
                    else if dist_y >= 2 && dist_x == 0 {
                        self.tail_y += 1;
                    }
                    else if dist_y >= 2 && dist_x == 1 {
                        self.tail_x += 1;
                        self.tail_y += 1;
                    }
                    
                },
                RopeMovement::Down => {
                    self.head_y -= 1;
                    let (dist_x, dist_y) = self.calc_distance();

                    if dist_y <= -2 && dist_x == -1 {
                        self.tail_x -= 1;
                        self.tail_y -= 1;
                    }
                    else if dist_y <= -2 && dist_x == 0 {
                        self.tail_y -= 1;
                    }
                    else if dist_y <= -2 && dist_x == 1 {
                        self.tail_x += 1;
                        self.tail_y -= 1;
                    }
                },
                RopeMovement::Left => {
                    self.head_x -= 1;
                    let (dist_x, dist_y) = self.calc_distance();
                    
                    if dist_x <= -2 && dist_y == -1 {
                        self.tail_x -= 1;
                        self.tail_y -= 1;
                    }
                    else if dist_x <= -2 && dist_y == 0 {
                        self.tail_x -= 1;
                    }
                    else if dist_x <= -2 && dist_y == 1 {
                        self.tail_x -= 1;
                        self.tail_y += 1;
                    }
                },
                RopeMovement::Right => {
                    self.head_x += 1;
                    let (dist_x, dist_y) = self.calc_distance();

                    if dist_x >= 2 && dist_y == -1 {
                        self.tail_x += 1;
                        self.tail_y -= 1;
                    }
                    else if dist_x >= 2 && dist_y == 0 {
                        self.tail_x += 1;
                    }
                    else if dist_x >= 2 && dist_y == 1 {
                        self.tail_x += 1;
                        self.tail_y += 1;
                    }
                },
            }
        }
        

    }

    // get the distance between the head and tail, separated in x and y
    fn calc_distance(&self) -> (i32, i32) {
        let dist_x = self.head_x - self.tail_x;
        let dist_y = self.head_y - self.tail_y;
        if dist_x > 2 || dist_x < -2 || dist_y > 2 || dist_y < -2 {
            println!("WARNING: This dist should not be possible: {dist_x} {dist_y}");
            println!("Full struct: {:?}", self);
            std::thread::sleep(time::Duration::from_millis(10000));
        }
        return ((dist_x),  (dist_y));
    }
}

pub fn day_9_main() {
    // https://adventofcode.com/2022/day/9
    let file_path = "inputs/day_9_rope.txt";
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    println!("\n{}\n", input);
    
    // so we now have a rope with a head and tail, the tail will follow the head as long as there are 2 spaces separating them. in a grid.
    // the square surrounding the head counts as 1 space, the tail will move 1 space to keep up, if that's not enough, it moves diagonally

    let mut test_rope = Rope {
        head_x: 0,
        head_y: 0,
        tail_x: -1,
        tail_y: -1,
    };

    println!("Test Rope: {:?}", test_rope);
    test_rope.move_head(&RopeMovement::Right, 4);
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
            }
            None => {
                println!("This line is empty somehow.");
                None
            }
        };

        ins_chars.next();
        let quantity = ins_chars.collect::<String>().parse().expect(&format!("The 3rd+ char should be a number: {}", ins_str));
        instructions.push( Instruction { movement: movement.unwrap(), quantity: quantity });
    }

    println!("{:?}", instructions);

    // PART 1: count how many spaces goes the TAIL through
    let mut tail_trail = HashSet::new();
    let mut rope = Rope {head_x: 0, head_y: 0, tail_x: 0, tail_y: 0};
    for inst in instructions {
        for _ in 0..inst.quantity {
            // yea don't want to refactor, without this for i can't save the entire trail easily
            rope.move_head(&inst.movement, 1);
            tail_trail.insert((rope.tail_x, rope.tail_y));
        }

    }
    println!("The final hashset is: {:?}", tail_trail);
    println!("The ammount of trails is: {}", tail_trail.len());
    
    
}
