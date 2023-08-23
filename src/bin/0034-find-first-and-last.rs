/**
 * 34. Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target
 * value.
 *
 * If target is not found in the array, return [-1, -1].
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 *   Input: nums = [5,7,7,8,8,10], target = 8
 *   Output: [3,4]
 *
 * Example 2:
 *   Input: nums = [5,7,7,8,8,10], target = 6
 *   Output: [-1,-1]
 *
 * Example 3:
 *   Input: nums = [], target = 0
 *   Output: [-1,-1]
 *
 *
 * Constraints:
 *   0 <= nums.length <= 10^5
 *   -10^9 <= nums[i] <= 10^9
 *   nums is a non-decreasing array.
 *   -10^9 <= target <= 10^9
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let low = nums.partition_point(|&v| v < target);
        if low == nums.len() || nums[low] != target {
            return vec![-1, -1];
        }

        let high = nums.partition_point(|&v| v <= target);
        vec![low as i32, high as i32 - 1]
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
