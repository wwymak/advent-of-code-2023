use std::fs;


fn get_prediction(input: Vec<i64>) -> (i64, i64) {

    let mut combined:Vec<Vec<i64>> = vec![input.clone()];
    
    let mut working = input.clone();
    while !working.iter().all(|x| x == &0) {
        let mut current:Vec<i64> = Vec::new();
        for n in 1..working.len() {
            current.push(working[n] - working[n -1]);
        }
        combined.push(current.clone());
        working = current.clone();
    } 

    let mut prediction = 0;
    let mut prediction_backwards = 0;

    for n in (0..combined.len()).rev() {
        let last = combined[n].last().unwrap();
        prediction += last;
    }  
    let mut current  = combined[combined.len() - 1 ].first().unwrap().clone();
    for n in (0..combined.len() - 1).rev() {
        let previous = combined[n].first().unwrap().clone();
        prediction_backwards = &previous - current;
        current = prediction_backwards.clone();
        // prediction_backwards += first;
    }  

    (prediction, prediction_backwards)
}

fn main() {

    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day9/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum:i64 = 0;
    let mut sum_backwards:i64 = 0;
    for line in contents.lines() {
        let entry:Vec<i64> = line.split_whitespace().map(|x| x.trim().parse::<i64>().unwrap()).collect();
        let preds= get_prediction(entry);

        sum += preds.0;
        sum_backwards += preds.1;
    }


    println!("{:?}, {}", sum, sum_backwards);
}
