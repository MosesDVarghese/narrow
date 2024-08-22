/*
Starter Code:
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
    }
}
*/

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 122;
const NAME: &str = "Best Time to Buy and Sell Stock II";
const VARIANT: &str = "Medium";
const TAKEAWAY: &str = "We don't always need multiple pointers. The knowledge of simply being given the whole array is adequate";

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        for i in 0..prices.len()-1 {
            if prices[i+1] > prices[i] {
                max_profit += prices[i+1] - prices[i];
            }
        }

        max_profit
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let prices: Vec<i32> = vec![7,1,5,3,6,4];
    // let prices: Vec<i32> = vec![1,2,3,4,5];
    // let prices: Vec<i32> = vec![7,6,4,3,1];
    let max_profit: i32 = Solution::max_profit(prices);
    println!("Max profit: {}", max_profit);

    info.end();
}