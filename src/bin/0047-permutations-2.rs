/**
 * 47. Permutations II
 *
 * Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
 *
 *
 * Example 1:
 *   Input: nums = [1,1,2]
 *   Output:
 *   [[1,1,2],
 *    [1,2,1],
 *    [2,1,1]]
 *
 * Example 2:
 *   Input: nums = [1,2,3]
 *   Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 *
 *
 * Constraints:
 *   1 <= nums.length <= 8
 *   -10 <= nums[i] <= 10
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn next_permutation(nums: &mut Vec<i32>) -> Result<(), ()> {
            let smaller_index = nums
                .windows(2)
                .enumerate()
                .rev()
                .find(|(_, chunk)| chunk[0] < chunk[1])
                .map(|(i, _)| i);

            if let Some(i) = smaller_index {
                let greater_index = nums
                    .iter()
                    .enumerate()
                    .rfind(|&(_, &v)| v > nums[i])
                    .map(|(j, _)| j)
                    .unwrap();
                nums.swap(i, greater_index);
                nums[i + 1..].reverse();

                Ok(())
            } else {
                Err(())
            }
        }

        nums.sort_unstable();
        let mut result = vec![nums.clone()];
        while let Ok(()) = next_permutation(&mut nums) {
            result.push(nums.clone());
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
        assert_eq!(Solution::permute_unique(vec![1, 1, 1]), vec![vec![1, 1, 1]]);

        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );

        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
