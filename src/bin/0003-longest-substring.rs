/**
 * 3. Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest
 * substring
 * without repeating characters.
 *
 *
 * Example 1:
 *   Input: s = "abcabcbb"
 *   Output: 3
 *   Explanation: The answer is "abc", with the length of 3.
 *
 * Example 2:
 *   Input: s = "bbbbb"
 *   Output: 1
 *   Explanation: The answer is "b", with the length of 1.
 *
 * Example 3:
 *   Input: s = "pwwkew"
 *   Output: 3
 *   Explanation: The answer is "wke", with the length of 3.
 *   Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *  
 *
 * Constraints:
 *   0 <= s.length <= 5 * 104
 *   s consists of English letters, digits, symbols and spaces.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // We have ASCII
        let mut seen_char_at = [0; 128];

        let mut max_length = 0;
        let mut start = 0;
        for (i, ch) in s.char_indices() {
            start = start.max(seen_char_at[ch as usize]);
            max_length = max_length.max(i - start + 1);
            seen_char_at[ch as usize] = i + 1;
        }

        max_length as i32
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabccbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
    }
}
