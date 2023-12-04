use std::fs;
use regex::Regex;



fn main() {
    let mut sum = 0;
    let mut sum2 = 0;
    let mut game_id = 0;
    let mut is_valid: bool  = true;
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day2/src/input.txt";
    for line in fs::read_to_string(file_path).unwrap().lines() {
        (game_id, is_valid) = get_combinations(line);
        let game_power = get_combinations_v2(line);
        if is_valid {
            sum += game_id;
        }
        sum2 += game_power;
    }
    println!("sum:{}", sum);
    println!("sum2:{}", sum2);
    
}

fn get_combinations(text: &str) -> (i32, bool) {
    let games: Vec<&str> = text.split(|c| c == ':' || c == ';').collect();
    let mut game_id: i32 = 0;
    let mut is_line_valid: bool = true;
    for (pos, game) in games.iter().enumerate() {
        if pos == 0{
            game_id = game.split(" ").last().unwrap().parse::<i32>().unwrap();

        } else {
            let ball_combos = game.split(',').collect::<Vec<&str>>();
            
            for ball_combo in ball_combos {
                let is_valid_combo = check_ball_count_valid(ball_combo);
                if is_valid_combo == false {
                    is_line_valid = false;
                }
            }
        }
    }

    return (game_id, is_line_valid);
}

fn get_combinations_v2(text: &str) -> i32 {
    let games: Vec<&str> = text.split(|c| c == ':' || c == ';').collect();
    let mut min_balls = vec![0,0,0]; // in order of blue, red, green

    for (pos, game) in games.iter().enumerate() {
        // for each game, split into the red, green, blue entries, find the number of balls and the color
        if pos == 0{
            continue;

        } else {
            
            let ball_combos = game.split(',').collect::<Vec<&str>>();
            let mut ball_count: i32;
            let mut ball_color: &str;

            for ball_combo in ball_combos {
                (ball_count, ball_color) = find_number_and_color(ball_combo);
                if (ball_color == "blue" ) & (ball_count > min_balls[0]){
                    min_balls[0] = ball_count;
                }
                if (ball_color == "red" ) & (ball_count > min_balls[1]){
                    min_balls[1] = ball_count;
                }
                if (ball_color == "green" ) & (ball_count > min_balls[2]){
                    min_balls[2] = ball_count;
                }
                
            }
        }
        
    }

    return min_balls[0]* min_balls[1] *min_balls[2];

}

fn check_ball_count_valid(text: &str) -> bool{
    // for q1: find the ball color and check if the number of balls is more than 
    // the allowed number
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


fn find_number_and_color(text: &str) -> (i32, &str) {
    let number_re = Regex::new(r"(?<number>\d+)").unwrap();
    let Some(caps) = number_re.captures(text) else {
        panic!("no number found");
    };
    let number = caps["number"].parse::<i32>().unwrap();

    let mut color:&str = "none";

    let blue = text.find("blue");
    let red = text.find("red");
    let green = text.find("green");

    if blue.is_some() {
        color = "blue";
    }
    else if red.is_some() {
        color = "red";
    }
    else if green.is_some() {
        color = "green";
    } else {
        panic!("no color found");
    }
    return (number, color);
}
// part 1: 2563
//part2 70768

