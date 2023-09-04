/**
 * 50. Pow(x, n)
 *
 * Implement pow(x, n), which calculates x raised to the power n (i.e., xn).
 *
 *
 * Example 1:
 *
 * Input: x = 2.00000, n = 10
 * Output: 1024.00000
 *
 * Example 2:
 *
 * Input: x = 2.10000, n = 3
 * Output: 9.26100
 *
 * Example 3:
 *
 * Input: x = 2.00000, n = -2
 * Output: 0.25000
 * Explanation: 2-2 = 1/22 = 1/4 = 0.25
 *
 *
 * Constraints:
 *   -100.0 < x < 100.0
 *   -2^31 <= n <= 2^31-1
 *   n is an integer.
 *   Either x is not zero or n > 0.
 *   -104 <= xn <= 104
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut result = 1.0;
        let mut base = x;
        let mut exp = n as i64;

        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }

        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }

        result
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::my_pow(2., 10), 1024.);
        assert_eq!(Solution::my_pow(2.1, 3), 9.261);
        assert_eq!(Solution::my_pow(2., -2), 0.25);
    }
}
