/**
 * 78. Subsets
 *
 * Given an integer array nums of unique elements, return all possible subsets (the power set).
 *
 * The solution set must not contain duplicate subsets. Return the solution in any order.
 *
 *
 * Example 1:
 *   Input: nums = [1,2,3]
 *   Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 *
 * Example 2:
 *   Input: nums = [0]
 *   Output: [[],[0]]
 *
 *
 * Constraints:
 *   1 <= nums.length <= 10
 *   -10 <= nums[i] <= 10
 *   All the numbers of nums are unique.
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn subsets_inner(current: &mut Vec<i32>, nums: &[i32], result: &mut Vec<Vec<i32>>) {
            if nums.is_empty() {
                return;
            }

            current.push(nums[0]);
            result.push(current.clone());
            subsets_inner(current, &nums[1..], result);
            current.pop();
            subsets_inner(current, &nums[1..], result);
        }

        let mut result = vec![vec![]];
        subsets_inner(&mut vec![], &nums, &mut result);
        result
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        let mut sets = Solution::subsets(vec![1, 2, 3]);
        sets.sort();
        assert_eq!(
            sets,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ]
        );

        let mut sets = Solution::subsets(vec![0]);
        sets.sort();
        assert_eq!(sets, vec![vec![], vec![0]])
    }
}
