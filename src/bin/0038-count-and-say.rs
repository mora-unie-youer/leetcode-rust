/**
 * 38. Count and Say
 *
 * The count-and-say sequence is a sequence of digit strings defined by the recursive formula:
 *   countAndSay(1) = "1"
 *   countAndSay(n) is the way you would "say" the digit string from countAndSay(n-1), which is then converted into a
 * different digit string.
 * To determine how you "say" a digit string, split it into the minimal number of substrings such that each substring
 * contains exactly one unique digit. Then for each substring, say the number of digits, then say the digit. Finally,
 * concatenate every said digit.
 *
 * Given a positive integer n, return the nth term of the count-and-say sequence.
 *
 *
 * Example 1:
 *   Input: n = 1
 *   Output: "1"
 *   Explanation: This is the base case.
 *
 * Example 2:
 *   Input: n = 4
 *   Output: "1211"
 *   Explanation:
 *   countAndSay(1) = "1"
 *   countAndSay(2) = say "1" = one 1 = "11"
 *   countAndSay(3) = say "11" = two 1's = "21"
 *   countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
 *
 *
 * Constraints:
 *   1 <= n <= 30
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::iter::Peekable;

struct CountAndSay<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    peekable: Peekable<I>,
}

impl<I> Iterator for CountAndSay<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let current = match self.peekable.next() {
            Some(v) => v,
            None => return None,
        };

        let mut counter = 1;
        loop {
            match self.peekable.peek() {
                Some(v) if &current == v => {
                    self.peekable.next();
                    counter += 1;
                }
                _ => break,
            }
        }

        Some((counter, current))
    }
}

trait CountAndSayable: Iterator {
    fn count_and_say(self) -> CountAndSay<Self>
    where
        Self: Sized,
        Self::Item: PartialEq,
    {
        CountAndSay {
            peekable: self.peekable(),
        }
    }
}
impl<T> CountAndSayable for T where T: Iterator {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut iter: Box<dyn Iterator<Item = char>> = Box::new(std::iter::once('1'));
        for _ in 1..n {
            iter = Box::new(
                iter.count_and_say()
                    .flat_map(|(count, ch)| [(count as u8 + b'0') as char, ch]),
            );
        }

        iter.collect()
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(4), "1211");
    }
}
