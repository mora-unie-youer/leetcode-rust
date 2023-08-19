/**
 * 28. Find the Index of the First Occurrence in a String
 *
 * Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is
 * not part of haystack.
 *
 *
 * Example 1:
 *   Input: haystack = "sadbutsad", needle = "sad"
 *   Output: 0
 *   Explanation: "sad" occurs at index 0 and 6.
 *   The first occurrence is at index 0, so we return 0.
 *
 * Example 2:
 *   Input: haystack = "leetcode", needle = "leeto"
 *   Output: -1
 *   Explanation: "leeto" did not occur in "leetcode", so we return -1.
 *
 *
 * Constraints:
 *   1 <= haystack.length, needle.length <= 104
 *   haystack and needle consist of only lowercase English characters.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|v| v as i32).unwrap_or(-1)
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::str_str("sadbutsad".into(), "sad".into()), 0);
        assert_eq!(Solution::str_str("leetcode".into(), "leeto".into()), -1);
    }
}
