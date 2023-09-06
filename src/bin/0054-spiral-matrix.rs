/**
 * 54. Spiral Matrix
 *
 * Given an m x n matrix, return all elements of the matrix in spiral order.
 *
 *
 * Example 1:
 *   Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 *   Output: [1,2,3,6,9,8,7,4,5]
 *
 * Example 2:
 *   Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
 *   Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 *
 * Constraints:
 *   m == matrix.length
 *   n == matrix[i].length
 *   1 <= m, n <= 10
 *   -100 <= matrix[i][j] <= 100
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let half = std::cmp::min(rows / 2, cols / 2);
        (0..=half)
            .flat_map(|offset| {
                let rows = rows - offset * 2;
                let cols = cols - offset * 2;

                let up = (0..cols).map(move |x| (x + offset, offset));
                let right = (1..rows).map(move |y| (cols + offset - 1, y + offset));
                let down = (0..cols - 1)
                    .rev()
                    .map(move |x| (x + offset, rows + offset - 1));
                let left = (1..rows - 1).rev().map(move |y| (offset, y + offset));

                up.chain(right).chain(down).chain(left)
            })
            .take(rows * cols)
            .map(|(x, y)| matrix[y][x])
            .collect()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
