/**
 * 59. Spiral Matrix II
 *
 * Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.
 *
 *
 * Example 1:
 *   Input: n = 3
 *   Output: [[1,2,3],[8,9,4],[7,6,5]]
 *
 * Example 2:
 *   Input: n = 1
 *   Output: [[1]]
 *
 *
 * Constraints:
 *   1 <= n <= 20
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut matrix = vec![vec![-1; n]; n];
        (0..=n / 2)
            .flat_map(|offset| {
                let size = n - offset * 2;

                let up = (0..size).map(move |x| (x + offset, offset));
                let right = (1..size).map(move |y| (size + offset - 1, y + offset));
                let down = (0..size - 1)
                    .rev()
                    .map(move |x| (x + offset, size + offset - 1));
                let left = (1..size - 1).rev().map(move |y| (offset, y + offset));

                up.chain(right).chain(down).chain(left)
            })
            .take(n * n)
            .enumerate()
            .for_each(|(i, (x, y))| matrix[y][x] = i as i32 + 1);

        matrix
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
