/**
 * 19. Remove Nth Node From End of List
 *
 * Given the head of a linked list, remove the nth node from the end of the list and return its head.
 *
 *
 * Example 1:
 *   Input: head = [1,2,3,4,5], n = 2
 *   Output: [1,2,3,5]
 *
 * Example 2:
 *   Input: head = [1], n = 1
 *   Output: []
 *
 * Example 3:
 *   Input: head = [1,2], n = 1
 *   Output: [1]
 *
 *
 * Constraints:
 *   The number of nodes in the list is sz.
 *   1 <= sz <= 30
 *   0 <= Node.val <= 100
 *   1 <= n <= sz
 *
 *
 * Follow up: Could you do this in one pass?
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        fn remove(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            match head {
                None => (None, 1),
                Some(mut node) => {
                    let (prev, i) = remove(node.next, n);
                    if i == n {
                        // Removing
                        (prev, i + 1)
                    } else {
                        // Making new next
                        node.next = prev;
                        (Some(node), i + 1)
                    }
                }
            }
        }

        remove(head, n).0
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
            Solution::remove_nth_from_end(create_list(vec![1, 2, 3, 4, 5]), 2),
            create_list(vec![1, 2, 3, 5])
        );

        assert_eq!(
            Solution::remove_nth_from_end(create_list(vec![1]), 1),
            create_list(vec![])
        );

        assert_eq!(
            Solution::remove_nth_from_end(create_list(vec![1, 2]), 1),
            create_list(vec![1])
        );
    }
}
