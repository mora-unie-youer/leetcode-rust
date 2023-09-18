/**
 * 67. Add Binary
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *
 *
 * Example 1:
 *   Input: a = "11", b = "1"
 *   Output: "100"
 *
 * Example 2:
 *   Input: a = "1010", b = "1011"
 *   Output: "10101"
 *
 *
 * Constraints:
 *   1 <= a.length, b.length <= 104
 *   a and b consist only of '0' or '1' characters.
 *   Each string does not contain leading zeros except for the zero itself.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let len = std::cmp::max(a.len(), b.len());

        let a = format!("{:0>len$}", a);
        let b = format!("{:0>len$}", b);

        let mut carry = 0;
        let mut result = String::new();
        for (c1, c2) in a.bytes().rev().zip(b.bytes().rev()) {
            let value = (c1 - b'0') + (c2 - b'0') + carry;
            carry = value / 2;
            result.push((value % 2 + b'0') as char);
        }

        if carry == 1 {
            result.push('1');
        }

        result.chars().rev().collect()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::add_binary("11".into(), "1".into()), "100");
        assert_eq!(Solution::add_binary("1010".into(), "1011".into()), "10101");
    }
}
