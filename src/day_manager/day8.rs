use std::fs;

#[derive(Debug)]
struct Tree {
    pos_x: usize,
    pos_y: usize,
    height: i32,
}

#[derive(Debug)]
struct Visibility {
    height: i32,
    pos_x: usize,
    pos_y: usize,
    highest_up: Tree,
    highest_down: Tree,
    highest_left: Tree,
    highest_right: Tree,
    blocker_up: Tree,
    blocker_down: Tree,
    blocker_left: Tree,
    blocker_right: Tree,
}

impl Visibility {
    fn is_visible(&self) -> bool {
        if self.height > self.highest_up.height
            || self.height > self.highest_down.height
            || self.height > self.highest_left.height
            || self.height > self.highest_right.height
        {
            return true;
        }
        return false;
    }

    fn calculate_vis_score(&self) -> usize {
        /*
        println!("{}", self.pos_y - self.blocker_up.pos_y);
        println!("{}", self.blocker_down.pos_y - self.pos_y);
        println!("{}", self.pos_x - self.blocker_left.pos_x);
        println!("{}", self.blocker_right.pos_x - self.pos_x);
        */
        return (self.pos_y - self.blocker_up.pos_y)
            * (self.blocker_down.pos_y - self.pos_y)
            * (self.pos_x - self.blocker_left.pos_x)
            * (self.blocker_right.pos_x - self.pos_x);
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
    // PART 2: what's the highest visibility score in the forest? doable by modifying part1.
    // score is: ammount of trees you can see from a tree (view blocked by same height+ trees) multiplied on the 4 dirs
    let res = count_visible_trees_and_vis_score(forest);
    println!("The results of parts 1 and 2 are: {:?}", res);

    // part 2 was indeed doable by adding features to part 1, I had to add structs to the visibility to keep track of the needed trees and add
    // a method to calculate the score, neat
}

fn parse_input_to_2dvec(input: String) -> Vec<Vec<i32>> {
    let mut forest: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        forest.push(
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse()
                        .expect(&format!("Could not parse character: {c}"))
                })
                .collect(),
        );
    }
    forest
}

fn count_visible_trees_and_vis_score(forest: Vec<Vec<i32>>) -> (i32, i32) {
    let mut visible = 0;
    let mut highest_vis_score = 0;
    // calculate visible trees of the middle square (no borders)
    // Y coords
    for j in 1..forest.len() - 1 {
        // X coords, we assume it's a square
        for i in 1..forest[0].len() - 1 {
            // check tree
            let curr_tree = forest[j][i];

            // build tree visibility struct with placeholder heights
            let mut curr_tree_visibility = Visibility {
                height: curr_tree,
                pos_x: i,
                pos_y: j,
                highest_up: Tree {
                    pos_x: 0,
                    pos_y: 0,
                    height: 0,
                },
                highest_down: Tree {
                    pos_x: 0,
                    pos_y: 0,
                    height: 0,
                },
                highest_left: Tree {
                    pos_x: 0,
                    pos_y: 0,
                    height: 0,
                },
                highest_right: Tree {
                    pos_x: 0,
                    pos_y: 0,
                    height: 0,
                },
                // for blockers we give them the border as the initial value
                blocker_up: Tree {
                    pos_x: i,
                    pos_y: 0,
                    height: forest[0][i],
                },
                blocker_down: Tree {
                    pos_x: i,
                    pos_y: forest.len() - 1,
                    height: forest[forest.len() - 1][i],
                },
                blocker_left: Tree {
                    pos_x: 0,
                    pos_y: j,
                    height: forest[j][0],
                },
                blocker_right: Tree {
                    pos_x: forest[0].len() - 1,
                    pos_y: j,
                    height: forest[j][forest[0].len() - 1],
                },
            };

            // check left
            let mut curr_check = 0;
            let mut compared_tree: &i32;
            while curr_check < i {
                compared_tree = &forest[j][curr_check];
                if curr_tree_visibility.highest_left.height <= *compared_tree {
                    curr_tree_visibility.highest_left.height = *compared_tree;
                    curr_tree_visibility.highest_left.pos_x = curr_check;
                    curr_tree_visibility.highest_left.pos_y = j;
                }
                if curr_tree_visibility.height <= *compared_tree {
                    curr_tree_visibility.blocker_left = Tree {
                        height: *compared_tree,
                        pos_x: curr_check,
                        pos_y: j,
                    };
                }
                curr_check += 1;
            }

            // check right
            curr_check = forest[0].len() - 1;
            while curr_check > i {
                compared_tree = &forest[j][curr_check];
                if curr_tree_visibility.highest_right.height <= *compared_tree {
                    curr_tree_visibility.highest_right.height = *compared_tree;
                    curr_tree_visibility.highest_right.pos_x = curr_check;
                    curr_tree_visibility.highest_right.pos_y = j;
                }
                if curr_tree_visibility.height <= *compared_tree {
                    curr_tree_visibility.blocker_right = Tree {
                        height: *compared_tree,
                        pos_x: curr_check,
                        pos_y: j,
                    };
                }
                curr_check -= 1;
            }

            // check up
            curr_check = 0;
            while curr_check < j {
                compared_tree = &forest[curr_check][i];
                if curr_tree_visibility.highest_up.height <= *compared_tree {
                    curr_tree_visibility.highest_up.height = *compared_tree;
                    curr_tree_visibility.highest_up.pos_x = i;
                    curr_tree_visibility.highest_up.pos_y = curr_check;
                }
                if curr_tree_visibility.height <= *compared_tree {
                    curr_tree_visibility.blocker_up = Tree {
                        height: *compared_tree,
                        pos_x: i,
                        pos_y: curr_check,
                    };
                }
                curr_check += 1;
            }

            // check down
            curr_check = forest.len() - 1;
            while curr_check > j {
                compared_tree = &forest[curr_check][i];
                if curr_tree_visibility.highest_down.height <= *compared_tree {
                    curr_tree_visibility.highest_down.height = *compared_tree;
                    curr_tree_visibility.highest_down.pos_x = i;
                    curr_tree_visibility.highest_down.pos_y = curr_check;
                }
                if curr_tree_visibility.height <= *compared_tree {
                    curr_tree_visibility.blocker_down = Tree {
                        height: *compared_tree,
                        pos_x: i,
                        pos_y: curr_check,
                    };
                }
                curr_check -= 1;
            }

            if curr_tree_visibility.is_visible() {
                visible += 1;
            }

            let vis_score = curr_tree_visibility.calculate_vis_score();
            if vis_score > highest_vis_score {
                highest_vis_score = vis_score;
            }

            println!(
                "New visibility for [{i}, {j}]: {:?}\n",
                curr_tree_visibility
            );
            println!("Visibility Score: {}\n", vis_score);
        }
    }

    // add the trees from the borders, it's just the len of the borders minus the 4 trees that overlap
    visible += forest.len() * 2 + forest[0].len() * 2 - 4;

    (
        visible.try_into().unwrap(),
        highest_vis_score.try_into().unwrap(),
    )
}
