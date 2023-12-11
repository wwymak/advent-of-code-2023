use std::fs;


fn get_prediction(input: Vec<i32>) -> i32 {

    let mut combined:Vec<Vec<i32>> = vec![input.clone()];
    
    let mut working = input.clone();
    while !working.iter().all(|x| x == &0) {
        let mut current:Vec<i32> = Vec::new();
        for n in 1..working.len() {
            current.push(working[n] - working[n -1]);
        }
        combined.push(current.clone());
        working = current.clone();
    } 

    let mut prediction = 0;

    for n in (0..combined.len()).rev() {
        let last = combined[n].last().unwrap();
        prediction += last;
    }   
    prediction
}

fn main() {

    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day9/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum:i32 = 0;
    for line in contents.lines() {
        let entry:Vec<i32> = line.split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect();
        sum += get_prediction(entry);
    }


    println!("{:?}", sum);
}
