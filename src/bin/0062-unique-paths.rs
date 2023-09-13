/**
 * 62. Unique Paths
 *
 * There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot
 * tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any
 * point in time.
 *
 * Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the
 * bottom-right corner.
 *
 * The test cases are generated so that the answer will be less than or equal to 2 * 10^9.
 *
 *
 * Example 1:
 *   Input: m = 3, n = 7
 *   Output: 28
 *
 * Example 2:
 *   Input: m = 3, n = 2
 *   Output: 3
 *   Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 *     1. Right -> Down -> Down
 *     2. Down -> Down -> Right
 *     3. Down -> Right -> Down
 *
 *
 * Constraints:
 *   1 <= m, n <= 100
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![1; n as usize]; m as usize];

        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[(m - 1) as usize][(n - 1) as usize]
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
