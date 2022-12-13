use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct HillNode {
    x: usize,
    y: usize,
    dist: i32,
}

#[derive(Debug)]
struct Hill {
    heights: HashMap<(usize, usize), u8>,
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn day_12_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/12
    let file_path = match input_file {
        Some(file) => file,
        None => "inputs/day_12_hills.txt".to_owned(),
    };
    let input = fs::read_to_string(&file_path).expect(&format!(
        "\n\nCould not read or find file: {}\n\n",
        file_path.clone()
    ));
    println!("\n{}\n", input);

    // search for the shortest path, a classic
    // each char is a height
    // you can only move to letters that are at most 1 letter higher
    // c->e invalid, c->d valid, z->a valid

    println!("testing hash map with tuple keys:");
    let mut hill_example = HashMap::new();
    hill_example.insert((0, 1), 123);
    println!("{:?}", hill_example);
    // key as a tuple works! nice

    // time for the real hill
    let hill_heights = HashMap::new();
    let mut hill = Hill {
        heights: hill_heights,
        max_x: 0,
        max_y: 0,
        start: (0, 0),
        end: (0, 0),
    };
    let letter_scores = get_scores_map();
    for (j, line) in input.lines().enumerate() {
        for (i, height) in line.chars().enumerate() {
            match height {
                'S' => hill.start = (i, j),
                'E' => hill.end = (i, j),
                _ => {}
            }
            hill.max_x = i;
            hill.heights.insert((i, j), letter_scores[&height]);
        }

        hill.max_y = j;
    }
    println!("The hill is: {:?}", hill);

    // not it's just a matter of doing a breath first search (BFS) on the hash map
    let mut queue_buf: VecDeque<u8> = VecDeque::new();
    queue_buf.push_front(hill.heights[&hill.start]);
    println!("{:?}", queue_buf);

    let part_1_steps = hill_breadth_first_search(&hill);
    println!("Steps taken in part 1: {}", part_1_steps);

    // part 2: change every a for an S to find the shortest path from a starting point that's an 'a' currently
    let mut shortest_path = part_1_steps;
    for j in 0..=hill.max_y {
        for i in 0..=hill.max_x {
            // 1 is 'a'
            if hill.heights[&(i, j)] == 1 {
                hill.heights.insert((hill.start.0, hill.start.1), 1);
                hill.heights.insert((i, j), 0);
                hill.start = (i, j);
                let new_steps = hill_breadth_first_search(&hill);
                println!("Current path: {}", new_steps);
                if new_steps < shortest_path && new_steps != -1 {
                    shortest_path = new_steps;
                }
            }
        }
    }

    println!("The new shortest path is: {}", shortest_path);
}

fn get_scores_map() -> HashMap<char, u8> {
    let mut scores = HashMap::new();
    for i in b'a'..=b'z' {
        scores.insert(i as char, i - 96);
    }
    scores.insert('S', 0);
    scores.insert('E', 27);
    scores
}

fn hill_breadth_first_search(hill: &Hill) -> i32 {
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new(); //  current node -> previous node
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_front(HillNode {
        x: hill.start.0,
        y: hill.start.1,
        dist: 0,
    });
    visited.insert((hill.start.0, hill.start.1));

    while queue.len() > 0 {
        //println!("QUEUE: {:?}", queue);
        let curr_node = queue.pop_back().unwrap();
        //println!("Visited: {:?}", visited);
        //println!("\nCurrent node: {:?}", curr_node);

        // arrived to the end
        if curr_node.x == hill.end.0 && curr_node.y == hill.end.1 {
            //println!("Finish!\nPrevious: {:?}\n", previous);
            //println!("Visited: {:?} ", visited);

            let mut traced_set: HashSet<(usize, usize)> = HashSet::new();
            println!("tracing previous:");

            let (mut a, mut b) = previous[&(hill.end.0, hill.end.1)];
            while a != hill.start.0 || b != hill.start.1 {
                traced_set.insert((a, b));
                print!("({}, {}) <- ", a, b);
                (a, b) = previous[&(a, b)];
            }
            traced_set.insert((a, b));
            println!("({}, {})", a, b);
            print_path(&traced_set, hill);

            return curr_node.dist;
        }

        for i in 0..4 {
            let (compared_x, compared_y);
            match i {
                // up
                0 => {
                    //println!("checking up");
                    // ignore if current node is at the top row
                    if curr_node.y == 0 {
                        //println!("skipping up");
                        continue;
                    }
                    (compared_x, compared_y) = (curr_node.x, curr_node.y - 1);
                }
                // down
                1 => {
                    //println!("checking down");
                    // ignore if current node is at the bottom row
                    if curr_node.y == hill.max_y {
                        //println!("skipping down");
                        continue;
                    }
                    (compared_x, compared_y) = (curr_node.x, curr_node.y + 1);
                }
                // left
                2 => {
                    //println!("checking left");
                    // ignore if current node is at the left column
                    if curr_node.x == 0 {
                        //println!("skipping left");
                        continue;
                    }
                    (compared_x, compared_y) = (curr_node.x - 1, curr_node.y);
                }
                // right
                3 => {
                    //println!("checking right");
                    // ignore if current node is at the right column
                    if curr_node.x == hill.max_x {
                        //println!("skipping right");
                        continue;
                    }
                    (compared_x, compared_y) = (curr_node.x + 1, curr_node.y);
                }
                _ => {
                    println!("THIS SHOULD NOT HAPPEN, PLEASE FIX: {:?}", curr_node);
                    compared_x = 0;
                    compared_y = 0;
                }
            }

            // or if the new node is too tall
            // 3rd condition: patch to make 0->2 work (i set 0 as start and should be same as 1 in weight)
            if hill.heights[&(compared_x, compared_y)] > hill.heights[&(curr_node.x, curr_node.y)]
                && hill.heights[&(compared_x, compared_y)]
                    - hill.heights[&(curr_node.x, curr_node.y)]
                    > 1
                && hill.heights[&(compared_x, compared_y)] != 2
            {
                //println!("Node too high! Skipping...");
                continue;
            }

            if !visited.contains(&(compared_x, compared_y)) {
                //println!("Adding: ({}, {}) -> ({}, {})", curr_node.x, curr_node.y, compared_x, compared_y);
                previous.insert((compared_x, compared_y), (curr_node.x, curr_node.y));
                queue.push_front(HillNode {
                    x: compared_x,
                    y: compared_y,
                    dist: curr_node.dist + 1,
                });
                visited.insert((compared_x, compared_y));
            }
        }
    }

    -1
}

fn print_path(map: &HashSet<(usize, usize)>, hill: &Hill) {
    println!("");
    for j in 0..=hill.max_y {
        print!("{} ", j + 1);

        for i in 0..=hill.max_x {
            if i == 0 && j == 0 {
                for k in 0..=hill.max_x + 1 {
                    print!("{} ", k);
                }
                print!("\n1 ")
            }

            if hill.start.0 == i && hill.start.1 == j {
                print!("S ");
            } else if hill.end.0 == i && hill.end.1 == j {
                print!("E ");
            } else if !map.contains(&(i, j)) {
                print!("  ");
            } else {
                print!("██");
            }
        }
        println!("");
    }
    println!("");
}
