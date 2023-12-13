use std::{fs, fmt::DebugTuple};
use itertools::Itertools;

fn manhattan_dist(x: Vec<i32>, y: Vec<i32>) -> i32 {
    (x[0] - y[0]).abs() + (x[1] - y[1]).abs()
}

fn expand_rows(row_coords:Vec<i32>, total_rows:i32, row_sep:i32) -> Vec<i32> {
    let mut idx = 0;
    let mut output:Vec<i32> = Vec::new();

    for i in 0..total_rows {
        if row_coords.contains(&i) {
            output.push(idx);
        } else {
            idx += row_sep
        }
    }

    output
}

fn main() {
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day11/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut galaxy:Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let linevec:Vec<char> = line.chars().collect();
        if !&linevec.contains(&'#') {
            galaxy.push(linevec.clone());
        }
        galaxy.push(linevec);
        

    }
    let mut empty_cols:Vec<i32> = Vec::new();
    for n in 0..galaxy[0].len() {
        let curr_col:Vec<char> = galaxy.iter().map(|x| x[n]).collect();
        if !curr_col.contains(&'#') {
            empty_cols.push(n as i32);
        }
    }

    let mut final_galaxy:Vec<Vec<char>> = Vec::new();
    for entry in &galaxy {
        let mut current = entry.clone();
        for (idx, col) in empty_cols.iter().enumerate() {
        
            current.insert((col.clone() as usize) + idx, '.');
            
        }
        final_galaxy.push(current);
    }

    let mut galaxy_coords:Vec<Vec<i32>> = Vec::new();

    for (idx, v )in final_galaxy.iter().enumerate() {
        for (vidx, element) in v.iter().enumerate() {
            if element == &'#' {
                galaxy_coords.push(vec![idx as i32, vidx as i32]);
            }
        }
    }

    println!("{:?}", galaxy_coords);

    let pair_ids:Vec<usize> = (0..galaxy_coords.len()).collect();

    let mut v = Vec::new();

    for (a, b) in pair_ids.into_iter().tuple_combinations() {
        v.push((a, b));
    }

    let distances:i32 = v.iter().map(|x| manhattan_dist(galaxy_coords[x.0].clone(), galaxy_coords[x.1].clone())).sum();

    println!("{:?}", distances);
    
}
