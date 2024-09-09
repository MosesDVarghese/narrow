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
