/*
Starter Code:
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    }
}
*/

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 26;
const NAME: &str = "Remove Duplicates from Sorted Array";
const VARIANT: &str = "Easy";

const TAKEAWAY: &str = "You can check the previous index with the current index";

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // tracker index
        let mut index = 1;
        // iterate through nums
        for i in 1..nums.len() {
            // if current num != num before it
            if nums[i] != nums[i - 1] {
                // replace index with curr val
                nums[index] = nums[i];
                // increment index
                index += 1;
            }
        }
        // return index
        index as i32
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    // let mut nums = vec![1, 1, 2];
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let len = Solution::remove_duplicates(&mut nums);
    println!("Array after removing duplicates: {:?}", &nums[..len as usize]);
    println!("New length: {}", len);

    info.end();
}