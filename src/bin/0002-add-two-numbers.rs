/**
 * 2. Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and
 * each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 *
 * Example 1:
 *   Input: l1 = [2,4,3], l2 = [5,6,4]
 *   Output: [7,0,8]
 *   Explanation: 342 + 465 = 807.
 *
 * Example 2:
 *   Input: l1 = [0], l2 = [0]
 *   Output: [0]
 *
 * Example 3:
 *   Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 *   Output: [8,9,9,9,0,0,0,1]
 *  
 *
 * Constraints:
 *   The number of nodes in each linked list is in the range [1, 100].
 *   0 <= Node.val <= 9
 *   It is guaranteed that the list represents a number that does not have leading zeros.
 */
pub struct Solution;
fn main() {}

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

// SUBMISSION CODE START

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = false;
        let mut head = Box::new(ListNode::new(0));
        let mut last = &mut head;

        let mut l1 = l1;
        let mut l2 = l2;
        while l1 != None || l2 != None || carry {
            let mut value = carry as i32;

            if let Some(n1) = l1 {
                value += n1.val;
                l1 = n1.next;
            }

            if let Some(n2) = l2 {
                value += n2.val;
                l2 = n2.next;
            }

            if value >= 10 {
                carry = true;
                value %= 10;
            } else {
                carry = false;
            }

            last.next = Some(Box::new(ListNode::new(value)));
            last = last.next.as_mut().unwrap();
        }

        head.next
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    fn create_list(digits: Vec<i32>) -> Option<Box<ListNode>> {
        let mut list = Box::new(ListNode::new(digits[0]));
        let mut last = &mut list;

        for digit in digits.into_iter().skip(1) {
            last.next = Some(Box::new(ListNode::new(digit)));
            last = last.next.as_mut().unwrap();
        }

        Some(list)
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::add_two_numbers(create_list(vec![2, 4, 3]), create_list(vec![5, 6, 4])),
            create_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(create_list(vec![0]), create_list(vec![0])),
            create_list(vec![0])
        );

        assert_eq!(
            Solution::add_two_numbers(
                create_list(vec![9, 9, 9, 9, 9, 9, 9]),
                create_list(vec![9, 9, 9, 9])
            ),
            create_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
