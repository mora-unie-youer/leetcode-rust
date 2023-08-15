/**
 * 23. Merge k Sorted Lists
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
 *
 * Merge all the linked-lists into one sorted linked-list and return it.
 *
 *
 * Example 1:
 *   Input: lists = [[1,4,5],[1,3,4],[2,6]]
 *   Output: [1,1,2,3,4,4,5,6]
 *   Explanation: The linked-lists are:
 *   [
 *     1->4->5,
 *     1->3->4,
 *     2->6
 *   ]
 *   merging them into one sorted list:
 *   1->1->2->3->4->4->5->6
 *
 * Example 2:
 *   Input: lists = []
 *   Output: []
 *
 * Example 3:
 *   Input: lists = [[]]
 *   Output: []
 *
 *
 * Constraints:
 *   k == lists.length
 *   0 <= k <= 104
 *   0 <= lists[i].length <= 500
 *   -104 <= lists[i][j] <= 104
 *   lists[i] is sorted in ascending order.
 *   The sum of lists[i].length will not exceed 104.
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

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut lists: BinaryHeap<_> = lists
            .into_iter()
            .filter(|list| list.is_some())
            .map(Reverse)
            .collect();

        while let Some(Reverse(mut list)) = lists.pop() {
            let next = list.as_mut().and_then(|curr| curr.next.take());
            if next.is_some() {
                lists.push(Reverse(next));
            }

            *tail = list;
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
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
            Solution::merge_k_lists(vec![
                create_list(vec![1, 4, 5]),
                create_list(vec![1, 3, 4]),
                create_list(vec![2, 6])
            ]),
            create_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );

        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
    }
}
