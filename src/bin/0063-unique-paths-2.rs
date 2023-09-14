/**
 * 63. Unique Paths II
 *
 * You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]).
 * The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or
 * right at any point in time.
 *
 * An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square
 * that is an obstacle.
 *
 * Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
 *
 * The testcases are generated so that the answer will be less than or equal to 2 * 109.
 *
 *
 * Example 1:
 *   Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 *   Output: 2
 *   Explanation: There is one obstacle in the middle of the 3x3 grid above.
 *     There are two ways to reach the bottom-right corner:
 *     1. Right -> Right -> Down -> Down
 *     2. Down -> Down -> Right -> Right
 *
 * Example 2:
 *   Input: obstacleGrid = [[0,1],[0,0]]
 *   Output: 1
 *
 *
 * Constraints:
 *   m == obstacleGrid.length
 *   n == obstacleGrid[i].length
 *   1 <= m, n <= 100
 *   obstacleGrid[i][j] is 0 or 1.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];

        for i in 0..m {
            if obstacle_grid[i][0] == 1 {
                break;
            }
            dp[i][0] = 1;
        }

        for j in 0..n {
            if obstacle_grid[0][j] == 1 {
                break;
            }
            dp[0][j] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 1 {
                    continue;
                }

                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
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
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
    }
}
