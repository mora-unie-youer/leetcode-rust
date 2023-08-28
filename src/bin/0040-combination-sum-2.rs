/**
 * 40. Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in
 * candidates where the candidate numbers sum to target.
 *
 * Each number in candidates may only be used once in the combination.
 *
 * Note: The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *   Input: candidates = [10,1,2,7,6,1,5], target = 8
 *   Output:
 *   [
 *     [1,1,6],
 *     [1,2,5],
 *     [1,7],
 *     [2,6]
 *   ]
 *
 * Example 2:
 *   Input: candidates = [2,5,2,1,2], target = 5
 *   Output:
 *   [
 *     [1,2,2],
 *     [5]
 *   ]
 *
 *
 * Constraints:
 *   1 <= candidates.length <= 100
 *   1 <= candidates[i] <= 50
 *   1 <= target <= 30
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::cmp::Ordering;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
                        if i != 0 && candidates[i - 1] == n {
                            continue;
                        }

                        current.push(n);
                        combination(&candidates[i + 1..], sum + n, target, current, answer);
                        current.pop();
                    }
                }
            }
        }

        candidates.sort_unstable();
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
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );

        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
