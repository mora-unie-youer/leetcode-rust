/**
 * 72. Edit Distance
 *
 * Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
 *
 * You have the following three operations permitted on a word:
 *   Insert a character
 *   Delete a character
 *   Replace a character
 *
 *
 * Example 1:
 *   Input: word1 = "horse", word2 = "ros"
 *   Output: 3
 *   Explanation:
 *   horse -> rorse (replace 'h' with 'r')
 *   rorse -> rose (remove 'r')
 *   rose -> ros (remove 'e')
 *
 * Example 2:
 *   Input: word1 = "intention", word2 = "execution"
 *   Output: 5
 *   Explanation:
 *   intention -> inention (remove 't')
 *   inention -> enention (replace 'i' with 'e')
 *   enention -> exention (replace 'n' with 'x')
 *   exention -> exection (replace 'n' with 'c')
 *   exection -> execution (insert 'u')
 *
 *
 * Constraints:
 *   0 <= word1.length, word2.length <= 500
 *   word1 and word2 consist of lowercase English letters.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let a = word1.as_bytes();
        let b = word2.as_bytes();

        let mut dp = vec![vec![0i32; b.len() + 1]; a.len() + 1];

        for (i, row) in dp.iter_mut().enumerate().skip(1) {
            row[0] = i as i32;
        }

        for (j, cell) in dp[0].iter_mut().enumerate().skip(1) {
            *cell = j as i32;
        }

        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    let insertion = 1 + dp[i][j - 1];
                    let deletion = 1 + dp[i - 1][j];
                    let replacement = 1 + dp[i - 1][j - 1];
                    dp[i][j] = insertion.min(deletion).min(replacement);
                }
            }
        }

        dp[a.len()][b.len()]
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
        assert_eq!(
            Solution::min_distance("intention".into(), "execution".into()),
            5
        );
    }
}
