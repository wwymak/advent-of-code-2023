use std::fs;
use std::collections::HashMap;
use num::Integer;

fn navigate(directions:&HashMap<&str, Vec<&str>>, instructions: &str, start: &str) -> i32{
    let mut num_steps = 0;

    let mut current = directions.get(start).unwrap();

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
            if next_key.ends_with("Z") {
                condition_met = true;
                break;
            }
            current = directions.get(next_key).unwrap();

        }
    }
    num_steps
}


fn navigate_parallel(directions:&HashMap<&str, Vec<&str>>, instructions: &str, start: Vec<&str>) -> i32{
    let mut num_steps = 0;

    let mut current:Vec<Vec<&str>> = start.clone().iter().map(|x| directions.get(x).unwrap().clone()).collect();

    let mut condition_met = false;

    while !condition_met{

        for i in instructions.chars() {
            let next_keys:Vec<&str> = match i == 'L' {
                false => current.clone().iter().map(|x| x[1]).collect(),
                true => current.clone().iter().map(|x| x[0]).collect(),
            };
            num_steps += 1;

            // if num_steps % 100_000 == 0 {
            //     println!("num steps: {}, {:?}", num_steps, next_keys)
            // }
            if next_keys.iter().all(|x| x.ends_with("Z")) {
                condition_met = true;
                break;
            }
            if next_keys.iter().any(|x| x.ends_with("Z")) {
                println!("num steps: {}, {:?}", num_steps, next_keys);
            }
            // if num_steps >= 20659 {
            //     println!("num steps: {}, {:?}", num_steps, next_keys);
            //     condition_met = true;
            //     break;
            // }
            current = next_keys.clone().iter().map(|x| directions.get(x).unwrap().clone()).collect();

        }
    }
    num_steps
}

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

    println!("{:?}", navigate_parallel(&directions, instructions, vec!["AAA"]));

    let starting_points: Vec<&str> = directions.keys().filter(|c| c.ends_with("A")).cloned().collect();

    let combos_vec:Vec<i32> = starting_points.iter().map(|x| navigate(&directions, instructions, x)).collect();

    let mut curr_lcm:i128 = combos_vec[0] as i128; 

    for i in 1..combos_vec.len() {
        curr_lcm = Integer::lcm(&curr_lcm, &(combos_vec[i] as i128));
    }

    println!("{:?}", curr_lcm);



   

}
