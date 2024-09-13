/*
Starter Code:

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {

    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 387;
const NAME: &str = "first unique character";
const VARIANT: &str = "easy";
const TAKEAWAY: &str = "";

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_map: HashMap<char, (i32, usize)> = HashMap::new();

        // Loop through the string to count occurrences and store the first index
        for (i, c) in s.chars().enumerate() {
            let entry = char_map.entry(c).or_insert((0, i));
            entry.0 += 1; // Increment the count of the character
        }

        // Find the first character with count 1 by looking at the stored indices
        for (i, c) in s.chars().enumerate() {
            if let Some(&(count, _)) = char_map.get(&c) {
                if count == 1 {
                    return i as i32;
                }
            }
        }

        -1
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let x: String = "example".to_string();
    // let x: i32 = -473;
    // let x: i32 = -1111222119;
    let y: i32 = Solution::first_uniq_char(x);
    println!("Index: {}", y);

    info.end();
}

/*
problem desc

Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.



Example 1:

Input: s = "leetcode"

Output: 0

Explanation:

The character 'l' at index 0 is the first character that does not occur at any other index.

Example 2:

Input: s = "loveleetcode"

Output: 2

Example 3:

Input: s = "aabb"

Output: -1



Constraints:

    1 <= s.length <= 105
    s consists of only lowercase English letters.



    */
