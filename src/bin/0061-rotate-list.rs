/**
 * 61. Rotate List
 *
 * Given the head of a linked list, rotate the list to the right by k places.
 *
 *
 * Example 1:
 *   Input: head = [1,2,3,4,5], k = 2
 *   Output: [4,5,1,2,3]
 *
 * Example 2:
 *   Input: head = [0,1,2], k = 4
 *   Output: [2,0,1]
 *
 *
 * Constraints:
 *     The number of nodes in the list is in the range [0, 500].
 *     -100 <= Node.val <= 100
 *     0 <= k <= 2 * 10^9
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
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// SUBMISSION CODE START

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let len = Self::length(&head);
        if len == 0 {
            return head;
        }

        let offset = k % len;

        if offset == 0 {
            return head;
        }

        let mut node = head.as_mut().unwrap();
        for _ in 0..len - offset - 1 {
            node = node.next.as_mut().unwrap();
        }

        let mut new_head = node.next.take().unwrap();

        let mut tail = &mut new_head;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = head;

        Some(new_head)
    }

    fn length(list: &Option<Box<ListNode>>) -> i32 {
        match list {
            None => 0,
            Some(list) => 1 + Self::length(&list.next),
        }
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    fn create_list(input: Vec<i32>) -> Option<Box<ListNode>> {
        if input.is_empty() {
            return None;
        }

        let mut list = Box::new(ListNode::new(input[0]));
        let mut last = &mut list;

        for element in input.into_iter().skip(1) {
            last.next = Some(Box::new(ListNode::new(element)));
            last = last.next.as_mut().unwrap();
        }

        Some(list)
    }

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::rotate_right(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![4, 5, 1, 2, 3])
        );
        assert_eq!(
            Solution::rotate_right(create_list(vec![0, 1, 2]), 4),
            create_list(vec![2, 0, 1])
        );
    }
}
