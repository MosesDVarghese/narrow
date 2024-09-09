/*
Starter Code:

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {

    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 326;
const NAME: &str = "is power of three";
const VARIANT: &str = "easy";
const TAKEAWAY: &str = "";

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // Create a mapping of Roman numeral symbols to their integer values.
        let mut roman_map = std::collections::HashMap::new();
        roman_map.insert('I', 1);
        roman_map.insert('V', 5);
        roman_map.insert('X', 10);
        roman_map.insert('L', 50);
        roman_map.insert('C', 100);
        roman_map.insert('D', 500);
        roman_map.insert('M', 1000);

        // Convert the string to a character vector.
        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;

        // Traverse the string.
        for i in 0..chars.len() {
            // Get the value of the current character.
            let curr_value = roman_map[&chars[i]];

            // If the current character is not the last one and is smaller than the next one, subtract it.
            if i < chars.len() - 1 && curr_value < roman_map[&chars[i + 1]] {
                total -= curr_value;
            } else {
                // Otherwise, add the value.
                total += curr_value;
            }
        }

        total
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let x = "LVXD".to_string();
    // let x: i32 = -473;
    // let x: i32 = -1111222119;
    let res = Solution::roman_to_int(x);
    println!("Res: {}", res);

    info.end();
}
