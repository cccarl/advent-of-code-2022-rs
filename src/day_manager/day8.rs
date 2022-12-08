use std::fs;


#[derive(Debug)]
struct Visibility {
    height: i32,
    highest_up: i32,
    highest_down: i32,
    highest_left: i32,
    highest_right: i32
}

impl Visibility {

    fn is_visible(&self) -> bool {
        if self.height > self.highest_up || self.height > self.highest_down || self.height > self.highest_left|| self.height > self.highest_right {
            return true;
        }
        return false;
    }

}

pub fn day_8_main() {
    // https://adventofcode.com/2022/day/8
    let file_path = "inputs/day_8_trees.txt";
    let input = fs::read_to_string(file_path).expect("Could not read or find file.");
    println!("\n{}\n", input);

    // ok seems simple enough. numbers are tree heights, check which trees are not visible on the x axis and y axis
    // visible means that there are only trees shorter than it up down left right. OR they are are the edge of the grid

    // first, parse into a vec of vecs
    let forest = parse_input_to_2dvec(input);

    // PART 1: count the visible trees
    let part1_res = count_visible_trees(forest);
    println!("The result of part 1 is: {}", part1_res);

}


fn parse_input_to_2dvec(input: String) -> Vec<Vec<i32>> {
    let mut forest: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        forest.push(line.chars().map(|c| c.to_string().parse().expect(&format!("Could not parse character: {c}"))).collect());
    }
    forest
}

fn count_visible_trees(forest: Vec<Vec<i32>>) -> i32 {

    let mut visible = 0;
    // calculate visible trees of the middle square (no borders)
    // Y coords
    for j in 1..forest.len()-1 {

        // X coords, we assume it's a square
        for i in 1..forest[0].len()-1 {

            // check tree
            let curr_tree = forest[j][i];

            // build tree visibility struct with placeholder heights
            let mut curr_tree_visibility = Visibility {
                height: curr_tree,
                highest_up: 0,
                highest_down: 0,
                highest_left: 0,
                highest_right: 0,
            };

            // check left
            let mut curr_check = 0;
            let mut compared_tree: &i32;
            while curr_check < i {
                compared_tree = &forest[j][curr_check];
                if curr_tree_visibility.highest_left < *compared_tree {
                    curr_tree_visibility.highest_left = *compared_tree;
                }
                curr_check += 1;
            }

            // check right
            curr_check = i+1;
            while curr_check < forest[0].len() {
                compared_tree = &forest[j][curr_check];
                if curr_tree_visibility.highest_right < *compared_tree  {
                    curr_tree_visibility.highest_right = *compared_tree;
                }
                curr_check += 1;
            }

            // check up
            curr_check = 0;
            while curr_check < j {
                compared_tree = &forest[curr_check][i];
                if curr_tree_visibility.highest_up < *compared_tree {
                    curr_tree_visibility.highest_up = *compared_tree;
                }
                curr_check +=1;
            }

            // check down
            curr_check = j+1;
            while curr_check < forest.len() {
                compared_tree = &forest[curr_check][i];
                if curr_tree_visibility.highest_down < *compared_tree {
                    curr_tree_visibility.highest_down = *compared_tree;
                }
                curr_check += 1;
            }

            if curr_tree_visibility.is_visible() {
                visible += 1;
            }

            println!("New visibility for [{i}, {j}]: {:?}", curr_tree_visibility);
        }
    }

    // add the trees from the borders, it's just the len of the borders minus the 4 trees that overlap
    visible += forest.len() * 2 + forest[0].len() * 2 - 4;

    visible.try_into().unwrap()
}