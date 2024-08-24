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
        /*
        this is a combination problem
        the goal is to find all possible combinations

        the question is, how do we find all possible combinations?

        to start, how do we find all possible combinations in real life?

        let's see, for example n=5

        2+2+1
        1+2+2
        2+1+2

        2+1+1+1
        1+2+1+1
        1+1+2+1
        1+1+1+2

        1+1+1+1+1

        something like this? y=8?
        so let's see how this works

        just for fun, let's try a formula

        5/1 + (5/2 % == 0) + 5/(2+1)

        so

        x1 = 5%1 == 0 ? 5/1 : 0;
        x2 = 5%2 == 0 ? 5/2 : 0;
        x3 = 5%(1+2) == 0 ? 5/(1+2) : 0;

        now let's try it out:
        x1 = 5
        x2 = 0
        x3 = 0

        another formula:
        x1 = 5-1 = 4
        x2 = 5-2 = 3
        x3 = 5-3 = 2

        total = 9

        okay this formula isn't working so let's see how we can solve this

        let's see this now 
        */
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let n: i32 = -1111222119;
    let y: i32 = Solution::climb_stairs(n);
    println!("New integer: {}", y);

    info.end();
}