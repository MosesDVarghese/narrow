// src/easy/problem1.rs

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
    let mut nums = vec![1, 1, 2];
    let len = Solution::remove_duplicates(&mut nums);
    println!("Array after removing duplicates: {:?}", &nums[..len as usize]);
    println!("New length: {}", len);
}