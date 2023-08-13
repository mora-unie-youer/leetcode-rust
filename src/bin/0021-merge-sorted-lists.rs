/**
 * 21. Merge Two Sorted Lists
 *
 * You are given the heads of two sorted linked lists list1 and list2.
 *
 * Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
 *
 * Return the head of the merged linked list.
 *
 *
 * Example 1:
 *   Input: list1 = [1,2,4], list2 = [1,3,4]
 *   Output: [1,1,2,3,4,4]
 *
 * Example 2:
 *   Input: list1 = [], list2 = []
 *   Output: []
 *
 * Example 3:
 *   Input: list1 = [], list2 = [0]
 *   Output: [0]
 *
 *
 * Constraints:
 *   The number of nodes in both lists is in the range [0, 50].
 *   -100 <= Node.val <= 100
 *   Both list1 and list2 are sorted in non-decreasing order.
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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut output = None;
        let mut output_tail = &mut output;

        loop {
            if list1.is_none() {
                std::mem::swap(output_tail, &mut list2);
                return output;
            }

            if list2.is_none() {
                std::mem::swap(output_tail, &mut list1);
                return output;
            }

            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                std::mem::swap(output_tail, &mut list1);
                output_tail = &mut output_tail.as_mut().unwrap().next;
                std::mem::swap(output_tail, &mut list1);
            } else {
                std::mem::swap(output_tail, &mut list2);
                output_tail = &mut output_tail.as_mut().unwrap().next;
                std::mem::swap(output_tail, &mut list2);
            }
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
            Solution::merge_two_lists(create_list(vec![1, 2, 4]), create_list(vec![1, 3, 4])),
            create_list(vec![1, 1, 2, 3, 4, 4])
        );

        assert_eq!(
            Solution::merge_two_lists(create_list(vec![]), create_list(vec![])),
            create_list(vec![])
        );

        assert_eq!(
            Solution::merge_two_lists(create_list(vec![]), create_list(vec![0])),
            create_list(vec![0])
        );
    }
}
