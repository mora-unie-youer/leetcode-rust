/**
 * 5. Longest Palindromic Substring
 *
 * Given a string S, return the longest palindromic substring in S.
 *
 * Example 1:
 *   Input: s = "babad"
 *   Output: "bab"
 *   Explanation: "aba" is also a valid answer.
 *
 * Example 2:
 *   Input: s = "cbbd"
 *   Output: "bb"
 *
 *  
 *
 * Constraints:
 *   1 <= s.length <= 1000
 *   s consist of only digits and English letters.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    fn is_palindrome(s: &&[u8]) -> bool {
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if s[i] != s[j] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }

    pub fn longest_palindrome(s: String) -> String {
        (1..=s.len())
            .rev()
            .find_map(|size| s.as_bytes().windows(size).find(Self::is_palindrome))
            .map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
            .unwrap()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::longest_palindrome("babad".into()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
    }
}
