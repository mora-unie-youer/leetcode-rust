/**
 * 7. Reverse Integer
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the
 * signed 32-bit integer range [-231, 231 - 1], then return 0.
 *
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 *  
 *
 * Example 1:
 *   Input: x = 123
 *   Output: 321
 *
 * Example 2:
 *   Input: x = -123
 *   Output: -321
 *
 * Example 3:
 *   Input: x = 120
 *   Output: 21
 *
 *
 * Constraints:
 *   -231 <= x <= 231 - 1
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let sign = x.signum();
        let mut x = if x < 0 { -x } else { x };
        let mut result: i32 = 0;
        while x > 0 {
            result = match result.checked_mul(10).and_then(|v| v.checked_add(x % 10)) {
                Some(v) => v,
                None => return 0,
            };

            x /= 10;
        }

        result * sign
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
