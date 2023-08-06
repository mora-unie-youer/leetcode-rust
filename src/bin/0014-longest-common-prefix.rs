/**
 * 14. Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 *
 * Example 1:
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 * Example 2:
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 * Constraints:
 *   1 <= strs.length <= 200
 *   0 <= strs[i].length <= 200
 *   strs[i] consists of only lowercase English letters.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut iterators: Vec<_> = strs.iter().map(|s| s.chars()).collect();

        let mut common_prefix = String::new();
        'main: while let Some(ch) = iterators[0].next() {
            for iter in iterators.iter_mut().skip(1) {
                match iter.next() {
                    Some(c) if ch == c => (),
                    _ => break 'main,
                }
            }

            common_prefix.push(ch);
        }

        common_prefix
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            "fl"
        );

        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            ""
        );
    }
}
