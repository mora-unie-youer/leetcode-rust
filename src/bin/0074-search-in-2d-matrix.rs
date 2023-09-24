/**
 * 74. Search a 2D Matrix
 *
 * You are given an m x n integer matrix matrix with the following two properties:
 *   Each row is sorted in non-decreasing order.
 *   The first integer of each row is greater than the last integer of the previous row.
 *
 * Given an integer target, return true if target is in matrix or false otherwise.
 *
 * You must write a solution in O(log(m * n)) time complexity.
 *
 *
 * Example 1:
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 *
 * Example 2:
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 *
 *
 * Constraints:
 *     m == matrix.length
 *     n == matrix[i].length
 *     1 <= m, n <= 100
 *     -10^4 <= matrix[i][j], target <= 10^4
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut low = 0;
        let mut high = rows * cols - 1;
        while low <= high {
            let mid = (low + high) / 2;
            let value = matrix[mid / cols][mid % cols];

            match value.cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => low = mid + 1,
                Ordering::Greater if mid == 0 => return false,
                Ordering::Greater => high = mid - 1,
            }
        }

        false
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert!(!Solution::search_matrix(vec![vec![1]], 0));
        assert!(!Solution::search_matrix(vec![vec![1, 1]], 0));
        assert!(!Solution::search_matrix(vec![vec![1], vec![1]], 0));
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));

        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }
}
