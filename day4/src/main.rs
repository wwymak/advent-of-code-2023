use std::fs;
use std::collections::HashSet;


fn main() {
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day4/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let base: i32 = 2;

    let mut sum = 0;

    for line in contents.lines() {
        let items: Vec<&str> = line.split(|c| c == ':' || c == '|').collect();
        let winning_str = items[1];
        let entry_str = items[2];

        let winning_numbers: Vec<i32> = winning_str.trim().split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        let entry_numbers: Vec<i32>  = entry_str.trim().split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();

        let winning_set: HashSet<i32> = HashSet::from_iter(winning_numbers);
        let entry_set: HashSet<i32> = HashSet::from_iter(entry_numbers);

        let num_matches = winning_set.intersection(&entry_set).count() as u32;
        if num_matches > 0 {
            let points: i32 = base.pow(num_matches -1);

            sum += points;
        }
    }

    println!("sum: {}", sum);
}
