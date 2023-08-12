/**
 * 20. Valid Parentheses
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *   Open brackets must be closed by the same type of brackets.
 *   Open brackets must be closed in the correct order.
 *   Every close bracket has a corresponding open bracket of the same type.
 *
 *
 * Example 1:
 *   Input: s = "()"
 *   Output: true
 *
 * Example 2:
 *   Input: s = "()[]{}"
 *   Output: true
 *
 * Example 3:
 *   Input: s = "(]"
 *   Output: false
 *
 *
 * Constraints:
 *   1 <= s.length <= 104
 *   s consists of parentheses only '()[]{}'.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    fn close_to_open(ch: char) -> char {
        match ch {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => unreachable!(),
        }
    }

    pub fn is_valid(s: String) -> bool {
        let mut parens = vec![];

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => parens.push(ch),

                ch if parens.last() == Some(&Self::close_to_open(ch)) => {
                    parens.pop();
                }

                _ => return false,
            }
        }

        parens.is_empty()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert!(Solution::is_valid("()".into()));
        assert!(Solution::is_valid("()[]{}".into()));
        assert!(!Solution::is_valid("(]".into()));
    }
}
