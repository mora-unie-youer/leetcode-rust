/**
 * 18. 4Sum
 *
 * Given an array nums of n integers, return an array of all the unique quadruplets
 * [nums[a], nums[b], nums[c], nums[d]] such that:
 *   0 <= a, b, c, d < n
 *   a, b, c, and d are distinct.
 *   nums[a] + nums[b] + nums[c] + nums[d] == target
 * You may return the answer in any order.
 *
 *
 * Example 1:
 *   Input: nums = [1,0,-1,0,-2,2], target = 0
 *   Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
 *
 * Example 2:
 *   Input: nums = [2,2,2,2,2], target = 8
 *   Output: [[2,2,2,2]]
 *
 *
 * Constraints:
 *   1 <= nums.length <= 200
 *   -10^9 <= nums[i] <= 10^9
 *   -10^9 <= target <= 10^9
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::cmp::Ordering;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = vec![];

        if nums.len() < 4 {
            return vec![];
        }

        let target = target as i64;
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }

            let a = nums[i];
            for j in i + 1..nums.len() - 2 {
                if j - i > 1 && nums[j - 1] == nums[j] {
                    continue;
                }

                let b = nums[j];
                let mut k = j + 1;
                let mut l = nums.len() - 1;
                while k < l {
                    let c = nums[k];
                    let d = nums[l];
                    let sum = a as i64 + b as i64 + c as i64 + d as i64;

                    match sum.cmp(&target) {
                        Ordering::Less => k += 1,
                        Ordering::Greater => l -= 1,
                        Ordering::Equal => {
                            result.push(vec![a, b, c, d]);

                            while k < l && nums[k] == c {
                                k += 1;
                            }
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
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );

        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );

        assert_eq!(
            Solution::four_sum(vec![-3, -1, 0, 2, 4, 5], 0),
            vec![vec![-3, -1, 0, 4]]
        );

        assert_eq!(
            Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]]
        );
    }
}
