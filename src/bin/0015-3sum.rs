/**
 * 15. 3Sum
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k,
 * and nums[i] + nums[j] + nums[k] == 0.
 *
 * Notice that the solution set must not contain duplicate triplets.
 *
 *
 * Example 1:
 *   Input: nums = [-1,0,1,2,-1,-4]
 *   Output: [[-1,-1,2],[-1,0,1]]
 *   Explanation:
 *   nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
 *   nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
 *   nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
 *   The distinct triplets are [-1,0,1] and [-1,-1,2].
 *   Notice that the order of the output and the order of the triplets does not matter.
 *
 * Example 2:
 *   Input: nums = [0,1,1]
 *   Output: []
 *   Explanation: The only possible triplet does not sum up to 0.
 *
 * Example 3:
 *   Input: nums = [0,0,0]
 *   Output: [[0,0,0]]
 *   Explanation: The only possible triplet sums up to 0.
 *
 *
 * Constraints:
 *   3 <= nums.length <= 3000
 *   -10^5 <= nums[i] <= 10^5
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = vec![];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            let a = nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let b = nums[j];
                let c = nums[k];
                let sum = a + b + c;

                match sum.cmp(&0) {
                    Ordering::Less => j += 1,
                    Ordering::Greater => k -= 1,
                    Ordering::Equal => {
                        result.push(vec![a, b, c]);

                        while j < k && nums[j] == b {
                            j += 1;
                        }
                    }
                }
            }
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
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );

        assert!(Solution::three_sum(vec![0, 1, 1]).is_empty());
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
