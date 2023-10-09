/**
 * 94. Binary Tree Inorder Traversal
 *
 * Given the root of a binary tree, return the inorder traversal of its nodes' values.
 *
 *
 * Example 1:
 * Input: root = [1,null,2,3]
 * Output: [1,3,2]
 *
 * Example 2:
 * Input: root = []
 * Output: []
 *
 * Example 3:
 * Input: root = [1]
 * Output: [1]
 *
 *
 * Constraints:
 *   The number of nodes in the tree is in the range [0, 100].
 *   -100 <= Node.val <= 100
 *
 *
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 */
pub struct Solution;
fn main() {}

// Definition for a binary tree node.
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// SUBMISSION CODE START

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut result = vec![];
        let mut stack = vec![(root, false)];

        while let Some((node, add)) = stack.pop() {
            if add {
                result.push(node.unwrap().borrow().val);
            } else if let Some(node) = node {
                let n = node.borrow();
                stack.push((n.right.clone(), false));
                stack.push((Some(node.clone()), true));
                stack.push((n.left.clone(), false));
            }
        }

        result
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Solution, TreeNode};

    #[test]
    fn test_examples() {
        let tree = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        };
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(tree)))),
            vec![1, 3, 2]
        );

        assert_eq!(Solution::inorder_traversal(None), vec![]);
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }
}
