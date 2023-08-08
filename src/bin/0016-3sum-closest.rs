/**
 * 16. 3Sum Closest
 *
 * Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest
 * to target.
 *
 * Return the sum of the three integers.
 *
 * You may assume that each input would have exactly one solution.
 *
 *
 * Example 1:
 *   Input: nums = [-1,2,1,-4], target = 1
 *   Output: 2
 *   Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 *
 * Example 2:
 *   Input: nums = [0,0,0], target = 1
 *   Output: 0
 *   Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
 *
 *
 * Constraints:
 *   3 <= nums.length <= 500
 *   -1000 <= nums[i] <= 1000
 *   -104 <= target <= 104
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut closest_diff = std::i32::MAX;
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

                let diff = sum - target;
                if diff.abs() < closest_diff.abs() {
                    closest_diff = diff;
                }

                match diff.signum() {
                    1 => k -= 1,
                    -1 => j += 1,
                    0 => return target,
                    _ => unreachable!(),
                }
            }
        }

        target + closest_diff
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
