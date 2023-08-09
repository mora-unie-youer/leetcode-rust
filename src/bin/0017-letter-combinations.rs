/**
 * 17. Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could
 * represent. Return the answer in any order.
 *
 * A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any
 * letters.
 *
 *
 * Example 1:
 *   Input: digits = "23"
 *   Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
 *
 * Example 2:
 *   Input: digits = ""
 *   Output: []
 *
 * Example 3:
 *   Input: digits = "2"
 *   Output: ["a","b","c"]
 *
 *
 * Constraints:
 *   0 <= digits.length <= 4
 *   digits[i] is a digit in the range ['2', '9'].
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    const BUTTONS: [&'static str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    fn generate_combination(combination: String, next_digits: &[u8], output: &mut Vec<String>) {
        if next_digits.is_empty() {
            output.push(combination);
        } else {
            let letters = Self::BUTTONS[(next_digits[0] - b'2') as usize];
            for letter in letters.chars() {
                let mut new_combination = combination.clone();
                new_combination.push(letter);
                Self::generate_combination(new_combination, &next_digits[1..], output);
            }
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut result = Vec::new();
        Self::generate_combination(String::new(), digits.as_bytes(), &mut result);
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
            Solution::letter_combinations("23".into()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".into()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations("2".into()),
            vec!["a", "b", "c"]
        );
    }
}
