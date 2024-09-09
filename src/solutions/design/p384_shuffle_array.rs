/*
Starter Code:

struct Solution {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {

    }

    fn reset(&self) -> Vec<i32> {

    }

    fn shuffle(&self) -> Vec<i32> {

    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 326;
const NAME: &str = "";
const VARIANT: &str = "";
const TAKEAWAY: &str = "";

use rand::seq::SliceRandom; // To shuffle the array
use rand::thread_rng; // For random number generation

pub struct Solution {
    original: Vec<i32>, // Store the original configuration
    current: Vec<i32>,  // Store the current (shuffled) configuration
}

impl Solution {
    // Initialize the Solution with the original array.
    fn new(nums: Vec<i32>) -> Self {
        Self {
            original: nums.clone(), // Clone the original array to keep a copy
            current: nums,          // The current array starts as the original array
        }
    }

    // Reset the array to its original configuration and return it.
    fn reset(&mut self) -> Vec<i32> {
        self.current = self.original.clone(); // Reset current to the original
        self.current.clone() // Return the reset array
    }

    // Shuffle the array randomly and return it.
    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = thread_rng(); // Create a random number generator
        self.current.shuffle(&mut rng); // Shuffle the current array in-place
        self.current.clone() // Return the shuffled array
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let mut obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    info.end();
}
