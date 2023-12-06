use std::fs;
use std::collections::{HashSet, HashMap};
use std::cmp;

fn main() {
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day4/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let base: i32 = 2;

    let mut sum = 0;
    let mut points_map: HashMap<u32, u32> = HashMap::new();
    let mut num_cards_map: HashMap<u32, u32> = HashMap::new();

    let lines: Vec<&str> = contents.lines().collect();
    let total_lines = lines.len() as u32;

    for n in 1..(total_lines + 1) {
        num_cards_map.insert(n, 1);
    }

    for (idx, line) in lines.iter().enumerate() {
        let items: Vec<&str> = line.split(|c| c == ':' || c == '|').collect();
        let card_number: u32 = idx as u32 + 1;
        let winning_str = items[1];
        let entry_str = items[2];

        let winning_numbers: Vec<i32> = winning_str.trim().split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();
        let entry_numbers: Vec<i32>  = entry_str.trim().split_whitespace().map(|c| c.parse::<i32>().unwrap()).collect();

        let winning_set: HashSet<i32> = HashSet::from_iter(winning_numbers);
        let entry_set: HashSet<i32> = HashSet::from_iter(entry_numbers);

        let num_matches = winning_set.intersection(&entry_set).count() as u32;
        points_map.insert(card_number, num_matches);

        let current_map = num_cards_map.clone();

        let num_copies_current_card = current_map.get(&card_number).unwrap();

        
        for n in (card_number + 1)..(cmp::min(card_number + num_matches + 1, total_lines)) {
            *num_cards_map.get_mut(&n).unwrap() += num_copies_current_card;
        }
        if num_matches > 0 {
            let points: i32 = base.pow(num_matches -1);

            sum += points;
        }
    }
    let total: u32 = num_cards_map.values().sum();
    println!("sum: {}", sum);
    println!("card number: {}", total);
}
