use core::num;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs;
use regex::Regex;

fn vec_to_set(vec: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(vec)
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Position {
    start: i32,
    end: i32,
    line_number: i32   ,
    value: i32,
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
    let re3 = Regex::new(r"(\*)").unwrap();

    let mut number_map: Vec<Position> = Vec::new();
    let mut symbol_map: Vec<Position> = Vec::new();
    let mut star_map: HashMap<Position, Vec<Position>> = HashMap::new();

    let lines = contents.lines(); 
    for (idx, line) in lines.enumerate() {
        for cap in re2.captures_iter(line) {
            let index = cap.iter().enumerate()               // skip the first group
                .find(|t| t.1.is_some())  // find the first `Some`
                .map(|t| t.0)             // extract the index
                .unwrap_or(0); 
            
            let curr_pos = Position {
                line_number: idx as i32,
                start: cap.get(index).unwrap().start() as i32,
                end: cap.get(index).unwrap().end() as i32,
                value: -1,
            };          
            symbol_map.push(curr_pos);
            
            // let positon = Position(line_number: index,start: cap.get(index).start)
        }

        for cap in re3.captures_iter(line) {
            let index = cap.iter().enumerate()               // skip the first group
                .find(|t| t.1.is_some())  // find the first `Some`
                .map(|t| t.0)             // extract the index
                .unwrap_or(0); 
            
            let curr_pos = Position {
                line_number: idx as i32,
                start: cap.get(index).unwrap().start() as i32,
                end: cap.get(index).unwrap().end() as i32,
                value: -1,
            };          
            star_map.insert(curr_pos, vec![]);
        }

        for cap in re.captures_iter(line) {
            let index = cap.iter().enumerate()               // skip the first group
                .find(|t| t.1.is_some())  // find the first `Some`
                .map(|t| t.0)             // extract the index
                .unwrap_or(0); 
            
            let curr_pos = Position {
                line_number: idx as i32,
                start: cap.get(index).unwrap().start() as i32,
                end: cap.get(index).unwrap().end() as i32,
                value: cap.get(index).unwrap().as_str().to_string().parse::<i32>().unwrap(),
            };          
            number_map.push(curr_pos);
        }
    }  
    println!("{}, {}, {}", number_map[0].line_number, number_map[0].start, number_map[0].end);
    println!("{}, {}, {}", symbol_map[0].line_number, symbol_map[0].start, symbol_map[0].end);
    let mut sum = 0;
    for number in number_map {
        // println!("number val: {}", number.value);
        
        let mut valid_symbols_positions = vec![];
        let mut valid_star_symbols_positions = vec![];

        let mut number_valid = false;
        for n in (number.start - 1)..(number.end + 1) {
            valid_symbols_positions.push(Position{
                line_number: number.line_number + 1,
                start: n,
                end: n + 1,
                value: -1
            });
            valid_symbols_positions.push(Position{
                line_number: number.line_number - 1,
                start: n,
                end: n + 1,
                value: -1
            })
        }
        valid_symbols_positions.push(Position{
            line_number: number.line_number,
            start: number.start -1,
            end: number.start,
            value: -1
        });
        valid_symbols_positions.push(Position{
            line_number: number.line_number,
            start: number.end ,
            end: number.end +1,
            value: -1
        });

        for n in (number.start )..(number.end - 1) {
            valid_star_symbols_positions.push(Position{
                line_number: number.line_number + 1,
                start: n,
                end: n + 1,
                value: -1
            });
            valid_star_symbols_positions.push(Position{
                line_number: number.line_number - 1,
                start: n,
                end: n + 1,
                value: -1
            })
        }

        valid_star_symbols_positions.push(Position{
            line_number: number.line_number,
            start: number.start -1,
            end: number.start,
            value: -1
        });
        valid_star_symbols_positions.push(Position{
            line_number: number.line_number,
            start: number.end ,
            end: number.end +1,
            value: -1
        });
        for item in valid_symbols_positions {
            if symbol_map.contains(&item) {
                number_valid = true;
            }
        }

        if number_valid {
            sum += number.value;
        }


        for item in valid_star_symbols_positions {
            let current = number.clone();
            match star_map.entry(item) {
                std::collections::hash_map::Entry::Vacant(e) => { e.insert(vec![current]); },
                std::collections::hash_map::Entry::Occupied(mut e) => { e.get_mut().push(current); }
            }
        }

        
    }

    println!("sum is {}", sum);

        
    let mut gearsum = 0;
    for (_key, value) in star_map.into_iter() {
        
        if value.len() == 2 {

            if (value[0].line_number == value[1].line_number) || ((value[0].line_number - value[1].line_number).abs() ==2) {

                let gear = value[0].value * value[1].value;
                gearsum += gear;
            }
        }
        
    }
    println!("gearsum is {}", gearsum);

    
}
