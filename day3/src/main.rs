use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs;
use regex::Regex;

fn vec_to_set(vec: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(vec)
}


fn main() {
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day3/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let unique_symbols = vec_to_set(contents.chars().filter(|c| (*c != '\n') & (*c != '.') & ( !c.is_digit(10)) ).collect());
    println!("{:?}", unique_symbols);

    let mut output_result = String::new().to_owned();
    unique_symbols.into_iter().for_each(|c| {
        let output = format!("\\{}{}", c, "|");
        output_result.push_str(&output);
    });
    output_result.pop();
    println!("{}", output_result);


    let re = Regex::new(r"(\d+)").unwrap();
    let re2 = Regex::new(&output_result).unwrap();
    let mut number_map: HashMap<i32, Vec<i32>> = HashMap::new();

    // let lines = contents.lines();   
    for cap in re2.captures_iter(".*..................") {
        let index = cap.iter().enumerate()
            // .skip(1)                  // skip the first group
            .find(|t| t.1.is_some())  // find the first `Some`
            .map(|t| t.0)             // extract the index
            .unwrap_or(0);            // get the index
        println!("group {:?}, match {:?}", index, cap.get(index));
    }
}
