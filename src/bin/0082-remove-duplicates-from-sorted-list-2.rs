/**
 * 82. Remove Duplicates from Sorted List II
 *
 * Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from
 * the original list. Return the linked list sorted as well.
 *
 *
 * Example 1:
 * Input: head = [1,2,3,3,4,4,5]
 * Output: [1,2,5]
 *
 * Example 2:
 * Input: head = [1,1,1,2,3]
 * Output: [2,3]
 *
 *
 * Constraints:
 *   The number of nodes in the list is in the range [0, 300].
 *   -100 <= Node.val <= 100
 *   The list is guaranteed to be sorted in ascending order.
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => match node.next {
                None => Some(node),
                Some(next_node) if next_node.val != node.val => {
                    node.next = Self::delete_duplicates(Some(next_node));
                    Some(node)
                }
                Some(mut next_node) => {
                    while next_node.val == node.val {
                        match next_node.next {
                            None => return None,
                            Some(n) => next_node = n,
                        }
                    }

                    Self::delete_duplicates(Some(next_node))
                }
            },
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
            Solution::delete_duplicates(create_list(vec![1, 2, 3, 3, 4, 4, 5])),
            create_list(vec![1, 2, 5])
        );
        assert_eq!(
            Solution::delete_duplicates(create_list(vec![1, 1, 1, 2, 3])),
            create_list(vec![2, 3])
        );
    }
}
