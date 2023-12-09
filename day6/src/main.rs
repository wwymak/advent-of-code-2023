use std::fs;

fn count_ways(max_time: u64, max_distance:u64) -> u64 {
    let mut ways = 0;
    for t in 0..max_time {
        let travelled = t * (max_time - t);
        if travelled > max_distance {
            ways += 1;
        }
    }
    return ways;
}
fn main() {
    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day6/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let times:Vec<&str> = lines[0].split(":").collect();
    let distance:Vec<&str> = lines[1].split(":").collect();
    let times2 : Vec<u64> = times[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let distance2 : Vec<u64> = distance[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

    // part 2 
    let times3 : Vec<String> = times[1].split_whitespace().map(|x| x.to_string()).collect();
    let distance3 : Vec<String> = distance[1].split_whitespace().map(|x| x.to_string()).collect();

    println!("{:?}", &times3);
    println!("{:?}", &distance3);

    // let distance3 = distance3.join("");
    // println!("{:?}",distance3);


    let time4: u64 = times3.join("").parse::<u64>().unwrap();
    let distance4:u64 = distance3.join("").parse::<u64>().unwrap();

    let mut ans: u64 = 1;

    for n in 0..distance2.len() {
        let time: u64 = times2[n];
        let distance: u64 = distance2[n];

        // time = t
        // button = t'
        // max distance = D
        // speed v = t'
        // distance travelled = t' * (t - t') > D 
        // let mut ways = 0;
        // for t in 0..time {
        //     let travelled = t * (time - t);
        //     if travelled > distance {
        //         ways += 1;
        //     }
        // }
        let ways = count_ways(time, distance);
        ans *= ways;
    }
    println!("Ans: {}", ans);

    let ans2 = count_ways(time4, distance4);
    println!("Ans2: {}", ans2);


    // let mut ways = 0;
    // for t in 0..time4 {
    //     let travelled = t * (time4 - t);
    //     if travelled > distance4 {
    //         ways += 1;
    //     }
    // }

    
}
