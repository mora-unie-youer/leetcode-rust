/**
 * 64. Minimum Path Sum
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum
 * of all numbers along its path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 *
 * Example 1:
 *   Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
 *   Output: 7
 *   Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
 *
 * Example 2:
 *   Input: grid = [[1,2,3],[4,5,6]]
 *   Output: 12
 *
 *
 * Constraints:
 *   m == grid.length
 *   n == grid[i].length
 *   1 <= m, n <= 200
 *   0 <= grid[i][j] <= 200
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];
        dp[0][0] = grid[0][0];

        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        for i in 1..n {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = grid[i][j] + std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
            }
        }

        dp[m - 1][n - 1]
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );

        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
