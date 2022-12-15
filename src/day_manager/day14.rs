use std::{collections::HashSet, fs, thread::sleep, time::Duration};

#[derive(Eq, PartialEq, Debug)]
struct CaveMap {
    used_spaces: HashSet<Point>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
    kind: PointKind,
    x: i32,
    y: i32,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum PointKind {
    Sand,
    Rock,
    Source,
}

pub fn day_14_main(input_file: Option<String>) {
    // https://adventofcode.com/2022/day/14
    let file_path = match input_file {
        Some(file) => file,
        None => "inputs/day_14_sand.txt".to_owned(),
    };
    let input = fs::read_to_string(&file_path).expect(&format!(
        "\n\nCould not read or find file: {}\n\n",
        file_path.clone()
    ));
    println!("\n{}\n", input);

    let mut cave = CaveMap {
        used_spaces: HashSet::new(),
        max_x: 500,
        max_y: 0,
        min_x: 500,
        min_y: 0,
    };

    for line in input.lines() {
        // yeah
        let line_points = line
            .split(" -> ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.split(',').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
            .iter()
            .map(|p| p.iter().map(|x| x.parse().unwrap()).collect())
            .collect::<Vec<Vec<i32>>>();

        //println!("{:?}", line_points);

        if line_points.len() > 1 {
            for i in 0..line_points.len() - 1 {
                //println!("CASE: {:?} vs {:?}", line_points[i], line_points[i+1]);
                // if Y increases, do Y1..Y2
                if line_points[i][1] < line_points[i + 1][1]
                    && line_points[i][0] == line_points[i + 1][0]
                {
                    for y in line_points[i][1]..=line_points[i + 1][1] {
                        (cave.max_x, cave.min_x, cave.max_y, cave.min_y) = get_min_max_nums(
                            cave.max_x,
                            cave.max_y,
                            cave.min_x,
                            cave.min_y,
                            line_points[i][0],
                            y,
                        );
                        cave.used_spaces.insert(Point {
                            x: line_points[i][0],
                            y,
                            kind: PointKind::Rock,
                        });
                    }
                }
                // if Y decreases, do Y2..Y1
                else if line_points[i][1] > line_points[i + 1][1]
                    && line_points[i][0] == line_points[i + 1][0]
                {
                    for y in line_points[i + 1][1]..=line_points[i][1] {
                        (cave.max_x, cave.min_x, cave.max_y, cave.min_y) = get_min_max_nums(
                            cave.max_x,
                            cave.max_y,
                            cave.min_x,
                            cave.min_y,
                            line_points[i][0],
                            y,
                        );
                        cave.used_spaces.insert(Point {
                            x: line_points[i][0],
                            y,
                            kind: PointKind::Rock,
                        });
                    }
                }
                // if X increases, do X1..X2
                else if line_points[i][0] < line_points[i + 1][0]
                    && line_points[i][1] == line_points[i + 1][1]
                {
                    for x in line_points[i][0]..=line_points[i + 1][0] {
                        (cave.max_x, cave.min_x, cave.max_y, cave.min_y) = get_min_max_nums(
                            cave.max_x,
                            cave.max_y,
                            cave.min_x,
                            cave.min_y,
                            x,
                            line_points[i][1],
                        );
                        cave.used_spaces.insert(Point {
                            x,
                            y: line_points[i][1],
                            kind: PointKind::Rock,
                        });
                    }
                }
                // if X decreases, do X2..X1
                else if line_points[i][0] > line_points[i + 1][0]
                    && line_points[i][1] == line_points[i + 1][1]
                {
                    for x in line_points[i + 1][0]..=line_points[i][0] {
                        (cave.max_x, cave.min_x, cave.max_y, cave.min_y) = get_min_max_nums(
                            cave.max_x,
                            cave.max_y,
                            cave.min_x,
                            cave.min_y,
                            x,
                            line_points[i][1],
                        );
                        cave.used_spaces.insert(Point {
                            x,
                            y: line_points[i][1],
                            kind: PointKind::Rock,
                        });
                    }
                }
                // the input is broken
                else {
                    println!("THE INPUT IS BROKEN or maybe it's me idk");
                    panic!("Please check the logic here, or the input");
                }
            }
        }
    }

    let sand_source = Point {
        kind: PointKind::Source,
        x: 500,
        y: 0,
    };
    cave.used_spaces.insert(sand_source.clone());

    //println!("{:?}", cave);
    println!("Rocks processed! This is the map: with the sand source:");

    // now it's time for the sand to fall
    // keep executing until the used spaces contain the spot below the source, this means that no more sand can fall
    println!("Sand is starting to fall:");
    let mut stalled = false;
    let inf_floor = true; // for part 2, just set to false to see part 1
    let mut sand_counter = 0;
    while !stalled {
        // create sand, do not add to actual cave until it can't move anymore
        let mut sand_falling = Point {
            kind: PointKind::Sand,
            x: sand_source.x,
            y: sand_source.y,
        };

        // sand falling loop
        loop {
            //println!("Curr sand pos: {} {}", sand_falling.x, sand_falling.y);
            // too slow for the full input
            //print_cave(&cave, &sand_falling, inf_floor);

            if sand_falling.y > cave.max_y + 1
                || cave.used_spaces.contains(&Point {
                    kind: PointKind::Sand,
                    x: sand_source.x,
                    y: sand_source.y,
                })
            {
                print_cave(&cave, &sand_falling, inf_floor);
                stalled = true;
                break;
            }

            // check the 3 spaces below the sand and act accordingly
            match count_occupied_below(&cave, &sand_falling, inf_floor) {
                1 => {
                    sand_falling.y += 1;
                    sand_falling.x -= 1;
                }
                2 => {
                    sand_falling.y += 1;
                    sand_falling.x += 1;
                }
                3 => {
                    cave.used_spaces.insert(sand_falling);
                    sand_counter += 1;
                    break;
                }
                _ => {
                    sand_falling.y += 1;
                }
            }
        }
    }

    println!(
        "Sand has stopped falling! The final sand block count is {}",
        sand_counter
    );
}

fn get_min_max_nums(
    og_max_x: i32,
    og_max_y: i32,
    og_min_x: i32,
    og_min_y: i32,
    new_x: i32,
    new_y: i32,
) -> (i32, i32, i32, i32) {
    (
        og_max_x.max(new_x),
        og_min_x.min(new_x),
        og_max_y.max(new_y),
        og_min_y.min(new_y),
    )
}

fn print_cave(cave: &CaveMap, sand_falling: &Point, inf_floor: bool) {
    let width_offset;
    if inf_floor {
        width_offset = 10;
    } else {
        width_offset = 2;
    }

    sleep(Duration::from_millis(50));
    for j in cave.min_y..=cave.max_y + 1 {
        for i in cave.min_x - width_offset..=cave.max_x + width_offset {
            if cave.used_spaces.contains(&Point {
                x: i,
                y: j,
                kind: PointKind::Rock,
            }) {
                print!("██");
            } else if cave.used_spaces.contains(&Point {
                x: i,
                y: j,
                kind: PointKind::Source,
            }) {
                print!("◥◤")
            } else if cave.used_spaces.contains(&Point {
                x: i,
                y: j,
                kind: PointKind::Sand,
            }) || sand_falling.x == i && sand_falling.y == j
            {
                print!("**");
            } else {
                print!("  ");
            }
        }
        println!("");
    }

    // extra line if the inf floor is active
    if inf_floor {
        for _ in cave.min_x - width_offset..=cave.max_x + width_offset {
            print!("██");
        }
    }

    println!("");
}

// always the same order: down, downleft, downright, so...
// all 3 used -> 3
// down and downleft -> 2
// down -> 1
// none -> 0
fn count_occupied_below(cave: &CaveMap, sand_falling: &Point, inf_floor: bool) -> i32 {
    // if the inf floor is active, just return 3 once it's reached
    if inf_floor && cave.max_y + 1 == sand_falling.y {
        return 3;
    }

    let mut count = 0;
    // check below
    if cave.used_spaces.contains(&Point {
        kind: PointKind::Rock,
        x: sand_falling.x,
        y: sand_falling.y + 1,
    }) || cave.used_spaces.contains(&Point {
        kind: PointKind::Sand,
        x: sand_falling.x,
        y: sand_falling.y + 1,
    }) || cave.used_spaces.contains(&Point {
        kind: PointKind::Source,
        x: sand_falling.x,
        y: sand_falling.y + 1,
    }) {
        count += 1;

        // check down left
        if cave.used_spaces.contains(&Point {
            kind: PointKind::Rock,
            x: sand_falling.x - 1,
            y: sand_falling.y + 1,
        }) || cave.used_spaces.contains(&Point {
            kind: PointKind::Sand,
            x: sand_falling.x - 1,
            y: sand_falling.y + 1,
        }) || cave.used_spaces.contains(&Point {
            kind: PointKind::Source,
            x: sand_falling.x - 1,
            y: sand_falling.y + 1,
        }) {
            count += 1;

            // check rown right
            if cave.used_spaces.contains(&Point {
                kind: PointKind::Rock,
                x: sand_falling.x + 1,
                y: sand_falling.y + 1,
            }) || cave.used_spaces.contains(&Point {
                kind: PointKind::Sand,
                x: sand_falling.x + 1,
                y: sand_falling.y + 1,
            }) || cave.used_spaces.contains(&Point {
                kind: PointKind::Source,
                x: sand_falling.x + 1,
                y: sand_falling.y + 1,
            }) {
                count += 1;
            }
        }
    }

    count
}
