use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn number_extractor(text: &str) -> i32 {
    let mut numerics = vec![];
    let mut output: i32 = 0;
    for b in text.chars() {
        match b.is_digit(10) {
            true => numerics.push((b.to_string()).parse::<i32>().unwrap()),
            false => (),
        }
    }
    if numerics.len() > 0 {
        output = numerics[0] *10 + numerics[numerics.len() - 1];
    }
    return output;

}
fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("/home/wwymak/code_experiments/advent_of_code_2023/day1/src/input1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // let is_numeric = ip.chars().map(|x| x.is_digit(10));

                sum += number_extractor(&ip);
                
                println!("{}", ip);
            }
        }
        println!("Sum: {}", sum);
    }
    else {
        println!("Error reading file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// ans: 55208