/*
Starter Code:

struct MinStack {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {

    }

    fn push(&self, val: i32) {

    }

    fn pop(&self) {

    }

    fn top(&self) -> i32 {

    }

    fn get_min(&self) -> i32 {

    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
*/

// #![allow(unused_variables)]
#![allow(dead_code)]

use crate::utils::problem_info::ProblemInfo;

const NUMBER: u16 = 155;
const NAME: &str = "min stack";
const VARIANT: &str = "medium";
const TAKEAWAY: &str = "";

struct MinStack {
    stack: Vec<i32>,     // Main stack to hold all elements
    min_stack: Vec<i32>, // Stack to hold the current minimums
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    // Initialize the stack object
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    // Push an element onto the stack
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        // Push the minimum value onto the min_stack
        if let Some(&min) = self.min_stack.last() {
            self.min_stack.push(std::cmp::min(min, val));
        } else {
            self.min_stack.push(val);
        }
    }

    // Pop the top element from the stack
    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    // Get the top element of the stack
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    // Retrieve the minimum element from the stack
    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

pub fn run() {
    let info = ProblemInfo::new(NUMBER, &NAME, &VARIANT, &TAKEAWAY);
    info.display();

    info.end();
}

/*
Problem Description:



Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the MinStack class:

    MinStack() initializes the stack object.
    void push(int val) pushes the element val onto the stack.
    void pop() removes the element on the top of the stack.
    int top() gets the top element of the stack.
    int getMin() retrieves the minimum element in the stack.

You must implement a solution with O(1) time complexity for each function.



Example 1:

Input
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

Output
[null,null,null,null,-3,null,0,-2]

Explanation
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin(); // return -3
minStack.pop();
minStack.top();    // return 0
minStack.getMin(); // return -2



Constraints:

    -231 <= val <= 231 - 1
    Methods pop, top and getMin operations will always be called on non-empty stacks.
    At most 3 * 104 calls will be made to push, pop, top, and getMin.

*/
