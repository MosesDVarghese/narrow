/*
Starter Code:

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {

    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 326;
const NAME: &str = "is power of three";
const VARIANT: &str = "easy";
const TAKEAWAY: &str = "this requires knowledge of math, 1162261467";

pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // The largest power of three that fits in a signed 32-bit integer
        const MAX_POWER_OF_THREE: i32 = 1162261467;

        // If n is greater than 0 and divides MAX_POWER_OF_THREE
        // without remainder, it's a power of three
        n > 0 && MAX_POWER_OF_THREE % n == 0
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let x: i32 = 27;
    // let x: i32 = -473;
    // let x: i32 = -1111222119;
    let res = Solution::is_power_of_three(x);
    println!("Res: {}", res);

    info.end();
}

/*

problem desc

Given an integer n, return true if it is a power of three. Otherwise, return false.

An integer n is a power of three, if there exists an integer x such that n == 3x.

 

Example 1:

Input: n = 27
Output: true
Explanation: 27 = 33

Example 2:

Input: n = 0
Output: false
Explanation: There is no x where 3x = 0.

Example 3:

Input: n = -1
Output: false
Explanation: There is no x where 3x = (-1).

 

Constraints:

    -231 <= n <= 231 - 1

 
Follow up: Could you solve it without loops/recursion?
*/