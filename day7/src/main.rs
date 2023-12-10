use core::num;
use std::fs;
use std::collections::HashMap;
use core::cmp::Ordering;

//  A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2 =-> highest strenght to lowest
// map to 14,13,12,11,10, 9...2


fn card_combo_to_hexadecimal(input:&str) -> i64 {

    let mut output:String= "".to_string();
    for c in input.chars() {

        if c.is_digit(10) {
            output += &c.to_string();
        } else {
            match c {
                'A' => {output += &format!("{:x}", 14)}
                'K' => {output += &format!("{:x}", 13)}
                'Q' => {output += &format!("{:x}", 12)}
                'J' => {output += &format!("{:x}", 11)}
                'T' => {output += &format!("{:x}", 10)}
                _ => {}
            }
        }
        
    }
    return i64::from_str_radix(&output, 16).unwrap();
}

fn card_combo_to_hexadecimal_v2(input:&str) -> i64 {
    //J card has lowest value
    let mut output:String= "".to_string();
    for c in input.chars() {

        if c.is_digit(10) {
            output += &c.to_string();
        } else {
            match c {
                'A' => {output += &format!("{:x}", 14)}
                'K' => {output += &format!("{:x}", 13)}
                'Q' => {output += &format!("{:x}", 12)}
                'J' => {output += &format!("{:x}", 1)}
                'T' => {output += &format!("{:x}", 10)}
                _ => {}
            }
        }
        
    }
    return i64::from_str_radix(&output, 16).unwrap();
}


#[derive(Debug, Eq, Ord, PartialEq)]
struct Card {
    combo: String,
    bet: i32,
    score: u32,
    group: u8
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.group == other.group {
            return Some(self.score.cmp(&other.score));
        }
        Some(self.group.cmp(&other.group))
    }
}
fn main() {
    

    println!("{}", &format!("{:x}", 1));
    let mut cards:Vec<Card> = Vec::new();
    let mut cards2:Vec<Card> = Vec::new();


    let file_path = "/home/wwymak/code_experiments/advent_of_code_2023/day7/src/input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    for line in contents.lines() {
        let mut chars_mapping:HashMap<char, u8> = HashMap::new();

        let (key, bet) = line.split_once(char::is_whitespace).unwrap();

        // combo_bet_mapping.insert(key, value.trim_start().parse::<u32>().unwrap());

        let base_score = card_combo_to_hexadecimal(key);
        let base_score2 = card_combo_to_hexadecimal_v2(key);
        
        for c in key.chars() {
            if let Some(x) = chars_mapping.get_mut(&c) {
                *x +=1;
            } else{
                chars_mapping.insert(c, 1);
            }
        }

        //1


        let mut group:u8 = 1;
        let mut group_with_joker = 1;


        //q1
        if chars_mapping.keys().len()  == 1 {
            group = 7;
        } else if chars_mapping.keys().len()  == 5 {
            group = 1;
        } else if chars_mapping.keys().len()  == 4 {
            group = 2;
        }else if (chars_mapping.keys().len()  == 3) & (*chars_mapping.values().max().unwrap() == 2) {
            group  =  3 ;
        } else if (chars_mapping.keys().len()  == 3) & (*chars_mapping.values().max().unwrap() == 3)  {
            group  =  4 ;
        }else if (chars_mapping.keys().len()  == 2) & (*chars_mapping.values().max().unwrap() == 3)  {
            group  =  5 ;
        }else if (chars_mapping.keys().len()  == 2) & (*chars_mapping.values().max().unwrap() == 4)  {
            group  =  6 ;
        }

        if !chars_mapping.contains_key(&'J') {
            println!("{:?}, {}", chars_mapping.keys(), key);
            group_with_joker = group
        } else {
            let num_keys_without_j = chars_mapping.keys().len() - 1;
            let num_js = chars_mapping.get(&'J').unwrap();

            let mut filtered:HashMap<char, u8> = HashMap::new();
            for (k, v) in chars_mapping.iter() {
                if k != &'J' {
                    filtered.insert(*k, *v);
                }
            }
            // chars_mapping.into_iter().filter(|(k, v)| *k=='J').collect();

            
            let mut max_count_per_key = 0;

            if *num_js == 5 as u8 {
                max_count_per_key = 5
            } else {
                max_count_per_key = filtered.values().max().unwrap() + num_js;
            }

            // let max_count_per_key = match num_js == 5{
            //     true
            //     _ =>
            //     false => todo!(), {
            //          = *filtered.values().max().unwrap() + num_js;
            //     }
            // }
            // let max_count_per_key = num_js if num_js == 5 else {*filtered.values().max().unwrap() + num_js;}

            if num_keys_without_j <= 1 {
                group_with_joker = 7;
            } else if num_keys_without_j  == 5 {
                group_with_joker = 1;
                println!("what:??? {}", key);
            } else if num_keys_without_j  == 4 {
                group_with_joker = 2;
            }else if (num_keys_without_j  == 3) & (max_count_per_key == 2) {
                group_with_joker  =  3 ;
            } else if (num_keys_without_j  == 3) & (max_count_per_key == 3)  {
                group_with_joker  =  4 ;
            }else if (num_keys_without_j == 2) & (max_count_per_key == 3)  {
                group_with_joker  =  5 ;
            }else if (num_keys_without_j == 2) & (max_count_per_key == 4)  {
                group_with_joker  =  6 ;
            }
        }


        


        cards.push(Card {
            combo:key.to_string(), 
            bet:bet.trim_start().parse::<i32>().unwrap(), 
            score:base_score as u32,
            group: group
         });

        cards2.push(Card {
            combo:key.to_string(), 
            bet:bet.trim_start().parse::<i32>().unwrap(), 
            score:base_score2 as u32,
            group: group_with_joker
        });

        

        

    }
    cards.sort();
    cards2.sort();

    let mut sum = 0;
    let mut sum2 = 0;

    for (idx, card) in cards.iter().enumerate() {
        sum += (idx as i32 + 1) * card.bet;
    }


    for (idx, card) in cards2.iter().enumerate() {
        sum2 += (idx as i32 + 1) * card.bet;

        println!("{:?}", card);
    }


    println!("sum: {}", sum);
    println!("sum2: {}", sum2);
}
