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

/*

problem desc

You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

Find and return the maximum profit you can achieve.

 

Example 1:

Input: prices = [7,1,5,3,6,4]
Output: 7
Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
Total profit is 4 + 3 = 7.

Example 2:

Input: prices = [1,2,3,4,5]
Output: 4
Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
Total profit is 4.

Example 3:

Input: prices = [7,6,4,3,1]
Output: 0
Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.

 

Constraints:

    1 <= prices.length <= 3 * 104
    0 <= prices[i] <= 104


    */