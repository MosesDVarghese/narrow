/*
Starter Code:

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        
    }
}
*/

#![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 344;
const NAME: &str = "Reverse String";
const VARIANT: &str = "Easy";
const TAKEAWAY: &str = "Approach was almost correct. But the while condition was overly complicated and wrong.";

pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // check if len is even or odd
        // if even, get halfway point of len
        // if odd, -1 and get halfway point of len

        // use 2 pointers that traverse front and back of array
        // swap front and back indexes
        // increment front index, decrement back index
        // do so until back index == front index OR back index and front index have difference of 1
        let mut f_index = 0;
        let mut b_index = s.len()-1;

        while f_index < b_index {
            let c = s[f_index];
            s[f_index] = s[b_index];
            s[b_index] = c;
            // or
            // s.swap(f_index, b_index);
            f_index += 1;
            b_index -= 1;
        }

    }
}

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    let mut s: Vec<char> = vec!['h','e','l','l','o'];
    Solution::reverse_string(&mut s);
    println!("New string: {:?}", s);

    info.end();
}

/*
problem desc

Write a function that reverses a string. The input string is given as an array of characters s.

You must do this by modifying the input array in-place with O(1) extra memory.

 

Example 1:

Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]

Example 2:

Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]

 

Constraints:

    1 <= s.length <= 105
    s[i] is a printable ascii character.


    */