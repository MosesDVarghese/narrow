#![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 24906;
const NAME: &str = "Name";
const VARIANT: &str = "Easy";
const TAKEAWAY: &str = "";

pub struct Solution;

impl Solution {
    pub fn func_name() -> i32 {
        let val: i32 = 6;

        val
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    // let x: i32 = 374;
    // let x: i32 = -473;
    // let x: i32 = -1111222119;
    let y: i32 = Solution::func_name();
    println!("New integer: {}", y);

    info.end();
}
