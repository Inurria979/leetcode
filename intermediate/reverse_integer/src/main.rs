use std::num::ParseIntError;

/*7. Reverse Integer
Medium
Topics
premium lock icon
Companies
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

 

Example 1:

Input: x = 123
Output: 321
Example 2:

Input: x = -123
Output: -321
Example 3:

Input: x = 120
Output: 21
 

Constraints:

-231 <= x <= 231 - 1
*/
pub struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x_str = x.to_string().chars().rev().collect::<String>();
        let mut flag = false;
        if x_str.contains("-"){
            x_str.pop();
            flag = true;
        }
    
        let mut x_i32 = x_str.parse::<i32>().unwrap_or(0);
        if flag {
            x_i32 *= -1;    
        }
        x_i32
    }
}

fn main(){
    Solution::reverse(1534236469);
}