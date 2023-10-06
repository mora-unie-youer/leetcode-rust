/**
 * 86. Partition List
 *
 * Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater
 * than or equal to x.
 *
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *
 *
 * Example 1:
 * Input: head = [1,4,3,2,5,2], x = 3
 * Output: [1,2,2,4,3,5]
 *
 * Example 2:
 * Input: head = [2,1], x = 2
 * Output: [1,2]
 *
 *
 * Constraints:
 *   The number of nodes in the list is in the range [0, 200].
 *   -100 <= Node.val <= 100
 *   -200 <= x <= 200
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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        // Dummy lists
        let mut before = ListNode::new(0);
        let mut after = ListNode::new(0);

        let mut before_tail = &mut before;
        let mut after_tail = &mut after;

        while let Some(mut node) = head {
            head = node.next.take();

            if node.val < x {
                before_tail.next = Some(node);
                before_tail = before_tail.next.as_mut().unwrap();
            } else {
                after_tail.next = Some(node);
                after_tail = after_tail.next.as_mut().unwrap();
            }
        }

        before_tail.next = after.next.take();
        before.next
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
            Solution::partition(create_list(vec![1, 4, 3, 2, 5, 2]), 3),
            create_list(vec![1, 2, 2, 4, 3, 5])
        );

        assert_eq!(
            Solution::partition(create_list(vec![2, 1]), 2),
            create_list(vec![1, 2])
        );
    }
}
