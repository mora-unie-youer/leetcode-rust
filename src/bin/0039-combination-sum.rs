/**
 * 39. Combination Sum
 *
 * Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of
 * candidates where the chosen numbers sum to target. You may return the combinations in any order.
 *
 * The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
 * frequency
 * of at least one of the chosen numbers is different.
 *
 * The test cases are generated such that the number of unique combinations that sum up to target is less than 150
 * combinations for the given input.
 *
 *
 * Example 1:
 *   Input: candidates = [2,3,6,7], target = 7
 *   Output: [[2,2,3],[7]]
 *   Explanation:
 *   2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
 *   7 is a candidate, and 7 = 7.
 *   These are the only two combinations.
 *
 * Example 2:
 *   Input: candidates = [2,3,5], target = 8
 *   Output: [[2,2,2,2],[2,3,3],[3,5]]
 *
 * Example 3:
 *   Input: candidates = [2], target = 1
 *   Output: []
 *
 *
 * Constraints:
 *   1 <= candidates.length <= 30
 *   2 <= candidates[i] <= 40
 *   All elements of candidates are distinct.
 *   1 <= target <= 40
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::cmp::Ordering;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn combination(
            candidates: &[i32],
            sum: i32,
            target: i32,
            current: &mut Vec<i32>,
            answer: &mut Vec<Vec<i32>>,
        ) {
            match sum.cmp(&target) {
                Ordering::Equal => answer.push(current.clone()),
                Ordering::Greater => (),
                Ordering::Less => {
                    for (i, &n) in candidates.iter().enumerate() {
                        current.push(n);
                        combination(&candidates[i..], sum + n, target, current, answer);
                        current.pop();
                    }
                }
            }
        }

        let mut answer = Vec::new();
        combination(&candidates, 0, target, &mut vec![], &mut answer);
        answer
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );

        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );

        assert!(Solution::combination_sum(vec![2], 1).is_empty());
    }
}
