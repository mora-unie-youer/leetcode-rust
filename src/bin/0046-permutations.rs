/**
 * 46. Permutations
 *
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
 *
 *
 * Example 1:
 *   Input: nums = [1,2,3]
 *   Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 *
 * Example 2:
 *   Input: nums = [0,1]
 *   Output: [[0,1],[1,0]]
 *
 * Example 3:
 *   Input: nums = [1]
 *   Output: [[1]]
 *
 *
 * Constraints:
 *   1 <= nums.length <= 6
 *   -10 <= nums[i] <= 10
 *   All the integers of nums are unique.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn permutations(
            nums: &[i32],
            visited: &mut usize,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == nums.len() {
                result.push(current.clone());
            }

            for (i, &num) in nums.iter().enumerate() {
                let bit = 1 << i;
                if *visited & bit != 0 {
                    continue;
                }

                *visited ^= bit;
                current.push(num);

                permutations(nums, visited, current, result);

                current.pop();
                *visited ^= bit;
            }
        }

        let mut result = Vec::with_capacity((1..=nums.len()).product());
        permutations(
            &nums,
            &mut 0,
            &mut Vec::with_capacity(nums.len()),
            &mut result,
        );

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
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
        assert_eq!(Solution::permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
