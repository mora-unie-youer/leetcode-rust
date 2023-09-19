/**
 * 69. Sqrt(x)
 *
 * Given a non-negative integer x, return the square root of x rounded down to the nearest integer.
 * The returned integer should be non-negative as well.
 *
 * You must not use any built-in exponent function or operator.
 *   For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
 *
 *
 * Example 1:
 *   Input: x = 4
 *   Output: 2
 *   Explanation: The square root of 4 is 2, so we return 2.
 *
 * Example 2:
 *   Input: x = 8
 *   Output: 2
 *   Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
 *
 *
 * Constraints:
 *   0 <= x <= 2^31 - 1
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::cmp::Ordering;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let (mut low, mut high) = (1, x);
        while low <= high {
            let mid = (low + high) / 2;

            match (x / mid).cmp(&mid) {
                Ordering::Equal => return mid,
                Ordering::Less => high = mid - 1,
                Ordering::Greater => low = mid + 1,
            }
        }

        high
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
