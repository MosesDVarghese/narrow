/*
[Problem Link](./problems/array/p26_remove_duplicates.md)

Starter Code:
impl Solution {
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

    }

}
*/

use crate::utils::problem_info::ProblemInfo;

pub const NUMBER: u16 = 26;
pub const NAME: &str = "Remove Duplicates from Sorted Array";
pub const VARIANT: &str = "Easy";

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[index] = nums[i];
                index += 1;
            }
        }
        index as i32
    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT);
    info.display();

    // let mut nums = vec![1, 1, 2];
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let len = Solution::remove_duplicates(&mut nums);
    println!("Array after removing duplicates: {:?}", &nums[..len as usize]);
    println!("New length: {}", len);
}