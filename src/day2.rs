use std::fs;

struct Game {
    opponent: char, // A, B, C -> RPS
    me: char        // X, Y, Z -> part 1: RPS ---- part 2: lose, tie, win 
}

enum Move {
    Rock,
    Paper,
    Scissor
}

// bonus points for part 2
fn bonus_points_enum(my_move: Move) -> i32 {
    match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3
    }
}

impl Game {

    // bonus points for part 1
    fn bonus_points_if_match(&self) -> Result<i32, String> {
        match self.me {
            'X' => Ok(1),
            'Y' => Ok(2),
            'Z' => Ok(3),
            _ => Err("Invalid move by me, from bonus points".to_string())
        }
    }

    // part 1 result, we assumed XYZ is RPS
    fn result_if_match(&self) -> Result<i32, String> {

        let bonus = self.bonus_points_if_match().unwrap();

        // win
        if self.me == 'X' && self.opponent == 'C' || self.me == 'Y' && self.opponent == 'A' || self.me == 'Z' && self.opponent == 'B'{
            return Ok(6 + bonus);
        }
        // draw
        else if self.me == 'X' && self.opponent == 'A' || self.me == 'Y' && self.opponent == 'B' || self.me == 'Z' && self.opponent == 'C'{
            return Ok(3 + bonus);
        }
        // lose
        else if self.me == 'X' && self.opponent == 'B' || self.me == 'Y' && self.opponent == 'C' || self.me == 'Z' && self.opponent == 'A'{
            return Ok(0 + bonus);
        }

        Err("Could not find a result for this game.".to_string())
    }

    // part 2 result, X means we have to lose, Y means we have to tie, Z means we have to win
    fn result_if_predicted_outcome(&self) -> Result<i32, String> {

        let my_move: Move;
        let outcome_points: i32;

        match self.me {
            // if we have to lose
            'X' => {
                outcome_points = 0;
                if self.opponent == 'A' {
                    my_move = Move::Scissor;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
                if self.opponent == 'B' {
                    my_move = Move::Rock;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
                if self.opponent == 'C' {
                    my_move = Move::Paper;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
            }
            // if we have to tie
            'Y' => {
                outcome_points = 3;
                if self.opponent == 'A' {
                    my_move = Move::Rock;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
                if self.opponent == 'B' {
                    my_move = Move::Paper;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
                if self.opponent == 'C' {
                    my_move = Move::Scissor;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
            }
            // if we have to win
            'Z' => {
                outcome_points = 6;
                if self.opponent == 'A' {
                    my_move = Move::Paper;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
                if self.opponent == 'B' {
                    my_move = Move::Scissor;
                    return Ok(outcome_points + bonus_points_enum(my_move));
                }
                if self.opponent == 'C' {
                    my_move = Move::Rock;
                    return Ok (outcome_points + bonus_points_enum(my_move));
                }
            }
            _ => return Err("Invalid match outcome, it wasn't X, Y or Z".to_string())
        }

        Err("Could not find a result for this game.".to_string())
    }
}

pub fn day_2_main() {
    // https://adventofcode.com/2022/day/2
    let file_path = "inputs/day_2_strategy.txt";

    let strategy_input = fs::read_to_string(file_path).expect("Could not read or find file");

    let test_game = Game {
        opponent: 'C',
        me: 'Z'
    };

    println!("The test result is: {}", test_game.result_if_match().unwrap());

    // time for the real input
    let mut final_sum = 0;
    for game in strategy_input.lines() {

        let mut game_to_chars = game.chars();

        let game_struct = Game {
            opponent: game_to_chars.nth(0).unwrap(),
            me: game_to_chars.nth(1).unwrap()
        };

        final_sum += game_struct.result_if_match().unwrap();
    }

    println!("Strategy guide result part 1: {}", final_sum);

    println!("\nPART 2: The XYZ Actually meant lose, tie, win.");

    let test_game = Game {
        opponent: 'C',
        me: 'Z',
    };
    println!("The result v2 of the test is: {}", test_game.result_if_predicted_outcome().unwrap());

    let mut final_sum = 0;
    for game in strategy_input.lines() {

        let mut game_to_chars = game.chars();

        let game_struct = Game {
            opponent: game_to_chars.nth(0).unwrap(),
            me: game_to_chars.nth(1).unwrap()
        };

        final_sum += game_struct.result_if_predicted_outcome().unwrap();
    }
    println!("Strategy guide result part 2: {}", final_sum);
    
}
