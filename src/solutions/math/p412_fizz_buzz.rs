/*
Starter Code:

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        
    }
}
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 412;
const NAME: &str = "Fizz Buzz";
const VARIANT: &str = "Easy";
const TAKEAWAY: &str = "Easy problem. Remember to check the if conditions and loop conditions. Make sure all 0..n and 0..=n are different! Make sure to account for unexpected data (such as negatives in this case).";

pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        // // define return array
        // let mut answer: Vec<String> = Vec::new(); 

        // // check for negatives
        // if n <= 0 {
        //     return answer;
        // }

        // // loop from 1 to the number
        // for i in 1..=n {
        //     // get the number
        //     println!("Number: {}", i);

        //     // if the number[i] is divisible by 3 and 5 -> fizzbuzz
        //     if i % 3 == 0 && i % 5 == 0 {
        //         answer.push("FizzBuzz".to_string());
        //     }
        //     // if the number[i] is divisible by 3 -> fizz
        //     else if i % 3 == 0 {
        //         answer.push("Fizz".to_string());
        //     }
        //     // if the number[i] is divisible by 5 -> buzz
        //     else if i % 5 == 0 {
        //         answer.push("Buzz".to_string());
        //     }
        //     else {
        //         answer.push(i.to_string());
        //     }
        // }

        // answer

        // alternate solution:

        // define the return array
        let mut answer: Vec<String> = Vec::with_capacity(n as usize);

        for i in 1..=n {
            // push with match statement
            answer.push(match(i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => i.to_string(),
            })
        }

        answer
    }
}

pub fn run() {
    let info: ProblemInfo = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let n: i32 = 15;
    // let n: i32 = -33;
    // let n: i32 = -1111222119;
    let answer: Vec<String> = Solution::fizz_buzz(n);
    println!("Result: {:?}", answer);

    info.end();
}
