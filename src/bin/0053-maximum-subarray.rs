/**
 * 53. Maximum Subarray
 *
 * Given an integer array nums, find the subarray with the largest sum, and return its sum.
 *
 *
 * Example 1:
 *   Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
 *   Output: 6
 *   Explanation: The subarray [4,-1,2,1] has the largest sum 6.
 *
 * Example 2:
 *   Input: nums = [1]
 *   Output: 1
 *   Explanation: The subarray [1] has the largest sum 1.
 *
 * Example 3:
 *   Input: nums = [5,4,-1,7,8]
 *   Output: 23
 *   Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
 *
 *
 * Constraints:
 *   1 <= nums.length <= 10^5
 *   -10^4 <= nums[i] <= 10^4
 *
 * Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach,
 * which is more subtle.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let mut current = 0;

        for num in nums {
            current += num;
            max = max.max(current);

            if current < 0 {
                current = 0;
            }
        }

        max
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
