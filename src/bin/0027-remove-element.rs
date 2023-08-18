/**
 * 27. Remove Element
 *
 * Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements
 * may be changed. Then return the number of elements in nums which are not equal to val.
 *
 * Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following
 * things:
 *   Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The
 * remaining elements of nums are not important as well as the size of nums.
 *   Return k.
 *
 *
 * Example 1:
 *   Input: nums = [3,2,2,3], val = 3
 *   Output: 2, nums = [2,2,_,_]
 *   Explanation: Your function should return k = 2, with the first two elements of nums being 2.
 *   It does not matter what you leave beyond the returned k (hence they are underscores).
 *
 * Example 2:
 *   Input: nums = [0,1,2,2,3,0,4,2], val = 2
 *   Output: 5, nums = [0,1,4,0,3,_,_,_]
 *   Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
 *   Note that the five elements can be returned in any order.
 *   It does not matter what you leave beyond the returned k (hence they are underscores).
 *
 *
 * Constraints:
 *   0 <= nums.length <= 100
 *   0 <= nums[i] <= 50
 *   0 <= val <= 100
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.swap_remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
    }
}
