/**
 * 22. Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 *
 * Example 1:
 * Input: n = 3
 * Output: ["((()))","(()())","(())()","()(())","()()()"]
 *
 * Example 2:
 * Input: n = 1
 * Output: ["()"]
 *
 *
 * Constraints:
 *   1 <= n <= 8
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn generate(prefix: &mut String, open: i32, close: i32, result: &mut Vec<String>) {
            if open == 0 && close == 0 {
                result.push(prefix.clone());
                return;
            }

            if open > 0 {
                prefix.push('(');
                generate(prefix, open - 1, close + 1, result);
                prefix.pop();
            }

            if close > 0 {
                prefix.push(')');
                generate(prefix, open, close - 1, result);
                prefix.pop();
            }
        }

        let mut result = Vec::new();
        generate(
            &mut String::with_capacity(2 * n as usize),
            n,
            0,
            &mut result,
        );

        result
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_owned(),
                "(()())".into(),
                "(())()".into(),
                "()(())".into(),
                "()()()".into()
            ]
        );

        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_owned()]);
    }
}
