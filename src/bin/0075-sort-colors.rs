/**
 * 75. Sort Colors
 *
 * Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are
 * adjacent, with the colors in the order red, white, and blue.
 *
 * We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
 *
 * You must solve this problem without using the library's sort function.
 *
 *
 * Example 1:
 *   Input: nums = [2,0,2,1,1,0]
 *   Output: [0,0,1,1,2,2]
 *
 * Example 2:
 *   Input: nums = [2,0,1]
 *   Output: [0,1,2]
 *
 *
 * Constraints:
 *   n == nums.length
 *   1 <= n <= 300
 *   nums[i] is either 0, 1, or 2.
 *
 *
 * Follow up: Could you come up with a one-pass algorithm using only constant extra space?
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0u16; 2];
        nums.iter()
            .filter(|&&v| v <= 1)
            .for_each(|&v| counts[v as usize] += 1);

        nums.iter_mut()
            .take(counts[0] as usize)
            .for_each(|v| *v = 0);
        nums.iter_mut()
            .skip(counts[0] as usize)
            .take(counts[1] as usize)
            .for_each(|v| *v = 1);
        nums.iter_mut()
            .skip((counts[0] + counts[1]) as usize)
            .for_each(|v| *v = 2);
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);

        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
