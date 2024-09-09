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

const NUMBER: u16 = 384;
const NAME: &str = "shuffle an array";
const VARIANT: &str = "medium";
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

/*
problem desc

Given an integer array nums, design an algorithm to randomly shuffle the array. All permutations of the array should be equally likely as a result of the shuffling.

Implement the Solution class:

    Solution(int[] nums) Initializes the object with the integer array nums.
    int[] reset() Resets the array to its original configuration and returns it.
    int[] shuffle() Returns a random shuffling of the array.



Example 1:

Input
["Solution", "shuffle", "reset", "shuffle"]
[[[1, 2, 3]], [], [], []]
Output
[null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]

Explanation
Solution solution = new Solution([1, 2, 3]);
solution.shuffle();    // Shuffle the array [1,2,3] and return its result.
                       // Any permutation of [1,2,3] must be equally likely to be returned.
                       // Example: return [3, 1, 2]
solution.reset();      // Resets the array back to its original configuration [1,2,3]. Return [1, 2, 3]
solution.shuffle();    // Returns the random shuffling of array [1,2,3]. Example: return [1, 3, 2]



Constraints:

    1 <= nums.length <= 50
    -106 <= nums[i] <= 106
    All the elements of nums are unique.
    At most 104 calls in total will be made to reset and shuffle.

* */
