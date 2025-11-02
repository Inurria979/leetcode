/*

Given a string containing digits from 2-9 inclusive, return all
possible letter combinations that the number could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons)
is given below. Note that 1 does not map to any letters.


Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
Example 2:

Input: digits = "2"
Output: ["a","b","c"]


Constraints:

1 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].
*/


const TWO: &str = "abc";
const THREE: &str = "def";
const FOUR: &str = "ghi";
const FIVE: &str = "jkl";
const SIX: &str = "mno";
const SEVEN: &str = "pqrs";
const EIGHT: &str = "tuv";
const NINE: &str = "wxyz";

pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        
        let mut letters: Vec<String> = vec![];
        for s in digits.chars() {
            match s {
                '2' => letters.push(TWO.to_string()),
                '3' => letters.push(THREE.to_string()),
                '4' => letters.push(FOUR.to_string()),
                '5' => letters.push(FIVE.to_string()),
                '6' => letters.push(SIX.to_string()),
                '7' => letters.push(SEVEN.to_string()),
                '8' => letters.push(EIGHT.to_string()),
                '9' => letters.push(NINE.to_string()),
                _ => println!(""),
            };
        }

        

        letters

    }
}

fn main() {

    println!("{:?}", Solution::letter_combinations("23".to_string()));
    
}
