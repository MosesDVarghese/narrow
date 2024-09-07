/*
Starter Code:

impl Solution {
    pub fn n_primes(n: i32) -> Vec<i32> {

    }
}
*/

#![allow(unused_variables)]
#![allow(dead_code)]

use std::usize;

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 5001;
const NAME: &str = "Sieve of Eratosthenes";
const VARIANT: &str = "Unknown";
const TAKEAWAY: &str = "it's simpler than others make it seem";

pub struct Solution;

impl Solution {
    pub fn n_primes(n: i32) -> Vec<i32> {
        let mut sieve = vec![1; n as usize];
        let mut res = Vec::new();

        for i in 2..n {
            if sieve[i as usize] == 1 {
                res.push(i);
            }

            let mut j = i * i;
            while j < n {
                sieve[j as usize] = 0;
                j += i;
            }
        }

        res
    }
}

pub fn run() {
    let info: ProblemInfo = ProblemInfo::new(NUMBER, NAME, VARIANT, TAKEAWAY);
    info.display();

    let n: i32 = 15;
    // let n: i32 = -33;
    // let n: i32 = -1111222119;
    let res: Vec<i32> = Solution::n_primes(n);
    println!("Result: {:?}", res);

    info.end();
}
