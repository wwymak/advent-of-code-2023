use std::fs;
use std::collections::HashMap;

fn main() {
    // let instructions = "RL";
    let instructions = "LRLRLLRRLRRRLRLRRLRLLRRLRRRLRLRLRLRRLRLLRRRLRRRLLRRLRRLRLRRRLLLRRLRLRLRLRLRLLRRRLRLRRRLRRRLRRRLRRRLRRRLRRRLRRRLRRLRRRLLRLLRRLRRLRRLRRRLLRLRRLRLRLRRLLRLRRRLRRLLRLRLRRRLRRLRRLRRLRLLRLRRRLLLRRRLLLLRRLRRRLLLRRLLRLRLRLLLRRRLLRRRLLLRLRRLLRRRLRRRLRLLRRRLRLRLRLLRRLLRRLRRRLRLRRRLRRLRLRRLRRRR";

    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day8/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut directions:HashMap<&str, Vec<&str>> = HashMap::new();

    for line in contents.lines() {
        let entries:Vec<&str> = line.split(|c| c == '=' || c == ',')
            .map(|x| x.trim().trim_end_matches(')').trim_start_matches('('))
            .collect();
        directions.insert(&entries[0], vec![&entries[1], &entries[2]]);
    }


    // println!("{:?}", directions);

    let mut num_steps = 0;

    let mut current = directions.get("AAA").unwrap();

    let mut condition_met = false;

    while !condition_met{

        for i in instructions.chars() {
            let next_key = match i == 'L' {
                // The arms of a match must cover all the possible values
                false => current[1],
                true => current[0],
                // TODO ^ Try commenting out one of these arms
            };
            num_steps += 1;
            if next_key == "ZZZ" {
                condition_met = true;
                break;
            }
            current = directions.get(next_key).unwrap();

        }
    }

    println!("num_steps {:?}", num_steps);

}
