/**
 * 58. Length of Last Word
 *
 * Given a string s consisting of words and spaces, return the length of the last word in the string.
 * A word is a maximal substring consisting of non-space characters only.
 *
 *
 * Example 1:
 *   Input: s = "Hello World"
 *   Output: 5
 *   Explanation: The last word is "World" with length 5.
 *
 * Example 2:
 *   Input: s = "   fly me   to   the moon  "
 *   Output: 4
 *   Explanation: The last word is "moon" with length 4.
 *
 * Example 3:
 *   Input: s = "luffy is still joyboy"
 *   Output: 6
 *   Explanation: The last word is "joyboy" with length 6.
 *
 *
 * Constraints:
 *   1 <= s.length <= 104
 *   s consists of only English letters and spaces ' '.
 *   There will be at least one word in s.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace().rev().next().unwrap().len() as i32
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::length_of_last_word("Hello World".into()), 5);
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".into()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".into()),
            6
        );
    }
}
