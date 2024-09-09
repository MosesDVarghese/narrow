/*
Starter Code:

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {

    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 70;
const NAME: &str = "Climbing Stairs";
const VARIANT: &str = "Easy";
const TAKEAWAY: &str = "";

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // formula for climbing stairs:
        // f(n) = f(n-1) + f(n-2)

        // deal with 1 and 2 cases
        if n == 1 {
            return 1
        }
        if n == 2 {
            return 2
        }

        // initialize variables
        let mut first: i32 = 1;
        let mut second: i32 = 2;
        let mut result: i32 = 0;

        for _ in 3..=n {
            result = first + second;
            first = second;
            second = result;
        }

        result
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let n: i32 = 6;
    let result: i32 = Solution::climb_stairs(n);
    println!("New integer: {}", result);

    info.end();
}

/*
problem desc

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

 

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps

Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step

 

Constraints:

    1 <= n <= 45


    */