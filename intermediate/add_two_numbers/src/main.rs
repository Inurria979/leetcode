/*
You are given two non-empty linked lists representing two non-negative integers.
the digits are stored in reverse order, and each of their nodes contains a single
digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero,
except the number 0 itself.

Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.
Example 2:

Input: l1 = [0], l2 = [0]
Output: [0]
Example 3:

Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]

Constraints:

The number of nodes in each linked list is in the range [1, 100].
0 <= Node.val <= 9
It is guaranteed that the list represents a number that does not have leading zeros.
*/


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let num1 = Self.get_num_from_list(l1);
        let num2 = Self.get_num_from_list(l2);

        println!("Num1: {:?} Num2 {:?}", num1, num2);
        Self.convert_num_to_list_reversed(num1 + num2)
    }

    fn get_num_from_list(self, l: Option<Box<ListNode>>) -> i32 {
        let mut num: i64 = 0;
        let mut i: i64 = 1;
        let mut cur = l.as_deref();
        while let Some(l) = cur {
            num += (l.val as i64) * i;
            i = i * 10;
            cur = l.next.as_deref();
        }

        num as i32
    }
    fn convert_num_to_list_reversed(self, num: i32) -> Option<Box<ListNode>> {
        let num_v: Vec<char> = num.to_string().chars().collect();
        const RADIX: u32 = 10;
        //println!("NUM {:?}", num_v);
        let mut current = None;
        for c in &num_v {
            let num = (*c).to_digit(RADIX).unwrap() as i32;
            //println!("Num {:?}", c);
            current = Some(Box::new(ListNode { 
                val: num as i32,
                next: current,
            }));
        }

        current
    }
}

fn main() {
    //test(Vec::from([2,4,3]), Vec::from([5,6,4]));
    test(Vec::from([9]), Vec::from([1,9,9,9,9,9,9,9,9,9]));

}

fn test(v1 : Vec<i32>, v2: Vec<i32>) {
    let num_v = Vec::from(v1);
    let mut l1 = None;
    for c in &num_v.clone() {    
        l1 = Some(Box::new(ListNode {
            val: *c,
            next: l1,
        }));
    }
    let num_vv = Vec::from(v2);
    let mut l2 = None;
    for c in &num_vv.clone() {    
        l2 = Some(Box::new(ListNode {
            val: *c,
            next: l2,
        }));
    }
    let sol = Solution::add_two_numbers(l1, l2);
    println!("Solution: {:?}", sol);
}
