use std::{fs, collections::{HashSet, HashMap}};
use itertools::Itertools;

fn manhattan_dist(x: Vec<i128>, y: Vec<i128>) -> i128 {
    (x[0] - y[0]).abs() + (x[1] - y[1]).abs()
}

fn expand_2d(row_coords:Vec<i128>, total_rows:i128, row_sep:i128) -> HashMap<i128, i128> {
    let mut idx = 0;
    let mut output:HashMap<i128, i128> = HashMap::new();

    for i in 0..total_rows {
        if row_coords.contains(&i) {
            output.insert(i, idx);
            idx +=1
        } else {
            idx += row_sep
        }
    }

    output
}

fn sum_distances(coords_x:Vec<i128>, coords_y:Vec<i128>) -> i128{
    println!("{}, {}", coords_x.len(), coords_y.len());
    let pair_ids:Vec<usize> = (0..coords_x.len()).collect();

    let mut v = Vec::new();

    for (a, b) in pair_ids.into_iter().tuple_combinations() {
        v.push((a, b));
    }

    let distances:i128 = v.iter().map(|x| 
        manhattan_dist(
        vec![coords_x[x.0], coords_y[x.0]], vec![coords_x[x.1], coords_y[x.1]])).sum();
    
    distances
}

fn main() {
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day11/src/input.txt";
    // let row_sep = 2; for case q1
    let row_sep = 1000000; //for q2
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut coords_x:Vec<i128> = Vec::new();
    let mut coords_y:Vec<i128> = Vec::new();

    let mut linecnt = 0;
    let mut colcnt = 0;

    for (idx, line) in contents.lines().enumerate() {
        linecnt +=1;
        let linevec:Vec<char> = line.chars().collect();
        colcnt = linevec.len() as i128;
        if linevec.contains(&'#') {
            
            for (idx2, entry) in linevec.iter().enumerate() {
                if entry == &'#' {
                    coords_x.push(idx2 as i128);
                    coords_y.push(idx as i128);
                }
            }
        }
    }

    let unique_x:HashSet<i128> = HashSet::from_iter(coords_x.clone());
    let unique_y:HashSet<i128> = HashSet::from_iter(coords_y.clone());

    let mut unique_sorted_x = Vec::from_iter(unique_x);
    unique_sorted_x.sort();
    let mut unique_sorted_y = Vec::from_iter(unique_y);
    unique_sorted_y.sort();

    let row_mapping = expand_2d(unique_sorted_y, linecnt,row_sep);
    let col_mapping = expand_2d(unique_sorted_x, colcnt,row_sep);

    let new_x:Vec<i128> = coords_x.iter().map(|x| col_mapping[x]).collect();
    let new_y:Vec<i128> = coords_y.iter().map(|x| row_mapping[x]).collect();

    let dist2 = sum_distances(new_x, new_y);
    println!("dist2: {}", dist2);
    
   
    
}
