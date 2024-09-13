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

const NUMBER: u16 = 13;
const NAME: &str = "roman to int";
const VARIANT: &str = "medium";
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

/*
problem desc

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer.



Example 1:

Input: s = "III"
Output: 3
Explanation: III = 3.

Example 2:

Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.

Example 3:

Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.



Constraints:

    1 <= s.length <= 15
    s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
    It is guaranteed that s is a valid roman numeral in the range [1, 3999].


 */
