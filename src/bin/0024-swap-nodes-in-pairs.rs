/**
 * 24. Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the
 * values in the list's nodes (i.e., only nodes themselves may be changed.)
 *
 *
 * Example 1:
 *   Input: head = [1,2,3,4]
 *   Output: [2,1,4,3]
 *
 * Example 2:
 *   Input: head = []
 *   Output: []
 *
 * Example 3:
 *   Input: head = [1]
 *   Output: [1]
 *
 *
 * Constraints:
 *   The number of nodes in the list is in the range [0, 100].
 *   0 <= Node.val <= 100
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });

        let mut pre = &mut dummy;
        while let Some(mut p) = pre.next.take() {
            if let Some(mut q) = p.next.take() {
                p.next = q.next.take();
                q.next = Some(p);
                pre.next = Some(q);

                pre = pre.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                pre.next = Some(p);
                pre = pre.next.as_mut().unwrap();
            }
        }

        dummy.next
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
            Solution::swap_pairs(create_list(vec![1, 2, 3, 4])),
            create_list(vec![2, 1, 4, 3])
        );

        assert_eq!(Solution::swap_pairs(None), None);

        assert_eq!(
            Solution::swap_pairs(create_list(vec![1])),
            create_list(vec![1])
        );
    }
}
