/**
 * 43. Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also
 * represented as a string.
 *
 * Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
 *
 *
 * Example 1:
 *   Input: num1 = "2", num2 = "3"
 *   Output: "6"
 *
 * Example 2:
 *   Input: num1 = "123", num2 = "456"
 *   Output: "56088"
 *
 *
 * Constraints:
 *   1 <= num1.length, num2.length <= 200
 *   num1 and num2 consist of digits only.
 *   Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let a: Vec<_> = num1.bytes().rev().map(|v| (v - b'0') as usize).collect();
        let b: Vec<_> = num2.bytes().rev().map(|v| (v - b'0') as usize).collect();

        let mut result = vec![0; a.len() + b.len()];

        // Multiply
        for i in 0..a.len() {
            for j in 0..b.len() {
                result[i + j] += a[i] * b[j];
            }
        }

        // Overflow
        for i in 0..result.len() - 1 {
            if result[i] >= 10 {
                result[i + 1] += result[i] / 10;
                result[i] %= 10;
            }
        }

        // Reduce
        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        result.reverse();
        result
            .into_iter()
            .map(|digit| (digit as u8 + b'0') as char)
            .collect()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::multiply("0".into(), "1".into()), "0");
        assert_eq!(Solution::multiply("2".into(), "3".into()), "6");
        assert_eq!(Solution::multiply("123".into(), "456".into()), "56088");
    }
}
