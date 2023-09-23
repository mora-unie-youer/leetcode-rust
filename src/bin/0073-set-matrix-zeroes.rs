/**
 * 73. Set Matrix Zeroes
 *
 * Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
 *
 * You must do it in place.
 *
 *
 * Example 1:
 *   Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 *   Output: [[1,0,1],[0,0,0],[1,0,1]]
 *
 * Example 2:
 *   Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 *   Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 *
 *
 * Constraints:
 *   m == matrix.length
 *   n == matrix[0].length
 *   1 <= m, n <= 200
 *   -231 <= matrix[i][j] <= 231 - 1
 *
 *
 * Follow up:
 *   A straightforward solution using O(mn) space is probably a bad idea.
 *   A simple improvement uses O(m + n) space, but still not the best solution.
 *   Could you devise a constant space solution?
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // We can use usize for bit storage -> 64 rows/cols in one number
        // Then we need 200 / 64 numbers -> 4 numbers
        let (mut rows_to_zero, mut cols_to_zero) = ([0usize; 4], [0usize; 4]);

        // Mask to get idnex
        const MASK: usize = (1 << 6) - 1;
        for (i, row) in matrix.iter().enumerate() {
            // Calculating index in rows array (i / 64)
            let row_index = i >> 6;

            for (j, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    // Calculating column index
                    let col_index = j >> 6;
                    // Setting bits
                    rows_to_zero[row_index] |= 1 << (i & MASK);
                    cols_to_zero[col_index] |= 1 << (j & MASK);
                }
            }
        }

        // Now we can zero
        for (i, row) in matrix.iter_mut().enumerate() {
            let row_index = i >> 6;

            if rows_to_zero[row_index] & (1 << (i & MASK)) != 0 {
                *row = vec![0; row.len()];
            } else {
                for (j, cell) in row.iter_mut().enumerate() {
                    let col_index = j >> 6;

                    if cols_to_zero[col_index] & (1 << (j & MASK)) != 0 {
                        *cell = 0;
                    }
                }
            }
        }
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
