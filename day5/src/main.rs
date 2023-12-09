use std::fs;
use std::collections::HashMap;



fn read_map_file(file_path: &str, input: &Vec<i64>) -> HashMap<i64, i64>{
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut mapping: HashMap<i64,i64> = HashMap::new();

    for line in contents.lines() {
        let info : Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let destination = info[0];
        let source = info[1];
        let range = info[2];

        let source_max = source + range;

        for entry in input {
            if (entry >= &source) & (entry < &source_max) {
                let diff = entry - source;
                let dest = destination + diff;
                mapping.insert(*entry, dest);
            }
        }
    }

    for entry in input {
        if !mapping.contains_key(&entry) {
            mapping.insert(*entry, *entry);
        }
    }

    return mapping;
}

fn hashmap_vals_to_vector(input: &HashMap<i64, i64>) -> Vec<i64> {
    let mut val_vec: Vec<i64> = Vec::new();
    for val in input.values() {
        val_vec.push(*val);
    }

    return val_vec
}
fn main() {
    let seeds: Vec<i64> = vec![
        1972667147,
        405592018, 1450194064, 27782252, 348350443, 61862174, 
        3911195009, 181169206, 626861593, 138786487, 2886966111 ,
        275299008, 825403564, 478003391, 514585599, 6102091, 2526020300, 15491453,
         3211013652, 546191739];

    let seed_soil = read_map_file(
        "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/seed-soil.txt", &seeds);

    
    let soil_fertise = read_map_file(
            "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/soil-fertiliser.txt", 
            &hashmap_vals_to_vector(&seed_soil));

    let fertiliser_water = read_map_file(
        "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/fertiliser-water.txt", 
        &hashmap_vals_to_vector(&soil_fertise));
    
    let water_light = read_map_file(
        "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/water-light.txt", 
        &hashmap_vals_to_vector(&fertiliser_water));
    
    let light_temp = read_map_file(
        "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/light-temp.txt", 
        &hashmap_vals_to_vector(&water_light));
    
    let temp_humidity = read_map_file(
        "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/temp-humidity.txt", 
        &hashmap_vals_to_vector(&light_temp));

    let humidity_location = read_map_file(
        "/home/wwymak/code_experiments/advent_of_code_2023/day5/src/inputs/humidity-location.txt", 
        &hashmap_vals_to_vector(&temp_humidity));

    let min_val = hashmap_vals_to_vector(&humidity_location);

    println!("{:?}", min_val.iter().min());
}
