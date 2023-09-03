/**
 * 49. Group Anagrams
 *
 * Given an array of strings strs, group the anagrams together. You can return the answer in any order.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the
 * original letters exactly once.
 *
 *
 * Example 1:
 *   Input: strs = ["eat","tea","tan","ate","nat","bat"]
 *   Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
 *
 * Example 2:
 *   Input: strs = [""]
 *   Output: [[""]]
 *
 * Example 3:
 *   Input: strs = ["a"]
 *   Output: [["a"]]
 *
 *
 * Constraints:
 *   1 <= strs.length <= 104
 *   0 <= strs[i].length <= 100
 *   strs[i] consists of lowercase English letters.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut letters = [0; 26];

            for ch in s.bytes() {
                letters[(ch - b'a') as usize] += 1;
            }

            anagrams.entry(letters).or_default().push(s);
        }

        anagrams.into_values().collect()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        // This tests will fail, but I just wanted to check for content
        assert_eq!(
            Solution::group_anagrams(vec![
                "eat".into(),
                "tea".into(),
                "tan".into(),
                "ate".into(),
                "nat".into(),
                "bat".into()
            ]),
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );

        assert_eq!(Solution::group_anagrams(vec!["".into()]), vec![vec![""]]);
        assert_eq!(Solution::group_anagrams(vec!["a".into()]), vec![vec!["a"]]);
    }
}
