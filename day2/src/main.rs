use std::fs;
use regex::Regex;



fn main() {
    let mut sum = 0;
    let mut game_id = 0;
    let mut is_valid: bool  = true;
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day2/src/input.txt";
    for line in fs::read_to_string(file_path).unwrap().lines() {
        (game_id, is_valid) = get_combinations(line);
        if is_valid {
            sum += game_id;
        }
    }
    println!("sum:{}", sum);
    
}

fn get_combinations(text: &str) -> (i32, bool) {
    let games: Vec<&str> = text.split(|c| c == ':' || c == ';').collect();
    // let game_id = games.iter().next().unwrap().chars().last().unwrap();
    // println!("Game {}:", game_id);
    let mut game_id: i32 = 0;
    let mut is_line_valid: bool = true;
    for (pos, game) in games.iter().enumerate() {
        if pos == 0{
            game_id = game.split(" ").last().unwrap().parse::<i32>().unwrap();
            println!("Game id: {}", game_id);

        } else {
            let ball_combos = game.split(',').collect::<Vec<&str>>();
            
            for ball_combo in ball_combos {
                let is_valid_combo = get_ball_color_and_count(ball_combo);
                println!("{}: {}", ball_combo, is_valid_combo);
                if is_valid_combo == false {
                    is_line_valid = false;
                }
            }
        }
        println!("Element at position {}: {:?}", pos, game);
    }

    return (game_id, is_line_valid);

    // println!("Game {}:", games[0]);
    // let game_id = games[0].chars().last().unwrap();
    // println!("Game {}:", game_id);
}



fn get_ball_color_and_count(text: &str) -> bool{
    let number_re = Regex::new(r"(?<number>\d+)").unwrap();
    let Some(caps) = number_re.captures(text) else {
        panic!("no number found");
    };
    let number = caps["number"].parse::<i32>().unwrap();

    let mut is_valid = true;

    let blue = text.find("blue");
    let red = text.find("red");
    let green = text.find("green");

    if blue.is_some() & (number > 14){
        is_valid = false;
    }
    if red.is_some() & (number > 12){
        is_valid = false;
    }
    if green.is_some() & (number > 13){
        is_valid = false;
    }

    return is_valid;
}

// part 1: 2563