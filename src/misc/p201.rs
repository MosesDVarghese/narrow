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
const NAME: &str = "Sieve of Eratosthenes | Primes";
const VARIANT: &str = "Unknown";
const TAKEAWAY: &str = "it's simpler than others make it seem";

pub struct Solution;

impl Solution {
    fn primes_less_than(n: i32) -> Vec<i32> {
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

    fn n_primes(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        res.push(2);
        let mut num = 3;

        while (res.len() as i32) < n {
            let mut is_prime: bool = true;
            for i in 0..res.len() {
                if num % res[i] == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                res.push(num);
            }

            num += 2;
        }

        res
    }
}

pub fn run() {
    let info: ProblemInfo = ProblemInfo::new(NUMBER, NAME, VARIANT, TAKEAWAY);
    info.display();

    let n: i32 = 15;
    let res: Vec<i32> = Solution::n_primes(n);
    println!("Result: {:?}", res);

    info.end();
}
