/**
 * 13. Roman to Integer
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol  Value
 * I        1
 * V        5
 * X        10
 * L        50
 * C        100
 * D        500
 * M        1000
 *
 * For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply
 * X + II. The number 27 is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII.
 * Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same
 * principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *   I can be placed before V (5) and X (10) to make 4 and 9.
 *   X can be placed before L (50) and C (100) to make 40 and 90.
 *   C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given a roman numeral, convert it to an integer.
 *
 *
 * Example 1:
 *   Input: s = "III"
 *   Output: 3
 *   Explanation: III = 3.
 *
 * Example 2:
 *   Input: s = "LVIII"
 *   Output: 58
 *   Explanation: L = 50, V= 5, III = 3.
 *
 * Example 3:
 *   Input: s = "MCMXCIV"
 *   Output: 1994
 *   Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *
 * Constraints:
 *   1 <= s.length <= 15
 *   s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
 *   It is guaranteed that s is a valid roman numeral in the range [1, 3999].
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        const LATIN_TO_INT: [i32; 26] = [
            0, 0, 100, 500, 0, 0, 0, 0, 1, 0, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10, 0, 0,
        ];

        let mut iter = s
            .bytes()
            .map(|ch| LATIN_TO_INT[(ch - b'A') as usize])
            .peekable();
        let mut result = 0;
        while let Some(digit) = iter.next() {
            let next = iter.peek();

            result += match next {
                Some(&v) if v > digit => -digit,
                _ => digit,
            };
        }

        result
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
    }
}
