/*
Starter Code:

impl Solution {
    pub fn count_primes(n: i32) -> i32 {

    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 204;
const NAME: &str = "Count Primes";
const VARIANT: &str = "Medium";
const TAKEAWAY: &str = "";

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0; // There are no primes less than 2
        }

        // Create a boolean vector to mark numbers as prime (true) or not (false)
        let mut is_prime: Vec<bool> = vec![true; n as usize];

        // 0 and 1 are not primes
        is_prime[0] = false;
        is_prime[1] = false;

        // Start marking multiples of each prime number as false
        for i in 2..((n as f64).sqrt() as usize + 1) {
            if is_prime[i] {
                // Mark multiples of i starting from i * i as false
                for j in (i * i..n as usize).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        // Count the number of primes (those marked true)
        is_prime.iter().filter(|&&p| p).count() as i32
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let x: i32 = 10;
    // let x: i32 = -473;
    // let x: i32 = -1111222119;
    let count: i32 = Solution::count_primes(x);
    println!("Primes: {}", count);

    info.end();
}

/*problem desc


Given an integer n, return the number of prime numbers that are strictly less than n.



Example 1:

Input: n = 10
Output: 4
Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.

Example 2:

Input: n = 0
Output: 0

Example 3:

Input: n = 1
Output: 0



Constraints:

    0 <= n <= 5 * 106

* */
