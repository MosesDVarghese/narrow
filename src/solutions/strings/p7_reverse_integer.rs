/*
Starter Code:

impl Solution {
    pub fn reverse(x: i32) -> i32 {

    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 344;
const NAME: &str = "Reverse Integer";
const VARIANT: &str = "Medium";
const TAKEAWAY: &str = "";

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        // // alternate faster solution
        // let mut x = x;
        // let mut reversed = 0;

        // while x != 0 {
        //     // Extract the last digit
        //     let digit = x % 10;

        //     // Prepare for the next iteration
        //     x /= 10;

        //     // Check for overflow before updating the reversed number
        //     if reversed > i32::MAX / 10 || (reversed == i32::MAX / 10 && digit > 7) {
        //         return 0; // Positive overflow
        //     }
        //     if reversed < i32::MIN / 10 || (reversed == i32::MIN / 10 && digit < -8) {
        //         return 0; // Negative overflow
        //     }

        //     // Update the reversed number
        //     reversed = reversed * 10 + digit;
        // }

        // reversed

        // Convert the integer to a string
        let mut x_str = x.to_string();

        // Check if the integer is negative
        let is_negative = x_str.starts_with('-');

        // If negative, remove the negative sign for processing
        if is_negative {
            x_str = x_str[1..].to_string();
        }

        // Reverse the string
        let reversed_str: String = x_str.chars().rev().collect();

        // Convert back to i32, checking for overflow
        match reversed_str.parse::<i32>() {
            Ok(mut reversed_num) => {
                if is_negative {
                    reversed_num = -reversed_num;
                }
                reversed_num
            }
            Err(_) => 0, // Return 0 if overflow occurs
        }
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let x: i32 = 374;
    // let x: i32 = -473;
    // let x: i32 = -1111222119;
    let y: i32 = Solution::reverse(x);
    println!("New integer: {}", y);

    info.end();
}

/*
problem desc

Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

 

Example 1:

Input: x = 123
Output: 321

Example 2:

Input: x = -123
Output: -321

Example 3:

Input: x = 120
Output: 21

 

Constraints:

    -231 <= x <= 231 - 1


    */