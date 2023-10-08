/**
 * 52. N-Queens II
 *
 * The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
 *
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *
 *
 * Example 1:
 * Input: n = 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
 *
 * Example 2:
 * Input: n = 1
 * Output: 1
 *
 *
 * Constraints:
 *   1 <= n <= 9
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn mark_cell(dp: &mut Vec<Vec<u8>>, row: usize, col: usize, old: u8, new: u8) {
            if dp[row][col] == old {
                dp[row][col] = new;
            }
        }

        fn mark_cell_range(dp: &mut Vec<Vec<u8>>, row: usize, col: usize, old: u8, new: u8) {
            for i in 1..dp.len() - row {
                if col >= i {
                    mark_cell(dp, row + i, col - i, old, new);
                }

                mark_cell(dp, row + i, col, old, new);

                if col + i < dp.len() {
                    mark_cell(dp, row + i, col + i, old, new);
                }
            }
        }

        // Storing 0 as free, 1 as queen, 1X as attacked from X row
        fn total_n_queens_inner(row: usize, dp: &mut Vec<Vec<u8>>, result: &mut usize) {
            if row == dp.len() {
                *result += 1;
                return;
            }

            for col in 0..dp.len() {
                if dp[row][col] == 0 {
                    // If position is free and not attacked, place queen here
                    dp[row][col] = 1;

                    // Mark all attacked cells
                    mark_cell_range(dp, row, col, 0, row as u8 + 10);

                    // Moving to the next row
                    total_n_queens_inner(row + 1, dp, result);

                    // Unmark all attacked cells
                    mark_cell_range(dp, row, col, row as u8 + 10, 0);

                    // Remove queen
                    dp[row][col] = 0;
                }
            }
        }

        let mut result = 0;
        let n = n as usize;
        total_n_queens_inner(0, &mut vec![vec![0; n]; n], &mut result);
        result as i32
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
