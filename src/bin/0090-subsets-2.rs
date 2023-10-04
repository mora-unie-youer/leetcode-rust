/**
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn subsets_with_dup_inner(
            nums: &[i32],
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if nums.is_empty() {
                result.push(current.clone());
                return;
            }

            let mut i = 1;
            while i < nums.len() && nums[i] == nums[i - 1] {
                i += 1;
            }

            for _ in 0..=i {
                subsets_with_dup_inner(&nums[i..], current, result);
                current.push(nums[0]);
            }

            for _ in 0..=i {
                current.pop();
            }
        }

        let mut result = vec![];
        nums.sort_unstable();
        subsets_with_dup_inner(&nums, &mut vec![], &mut result);
        result
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        let mut result = Solution::subsets_with_dup(vec![1, 2, 2]);
        result.sort_unstable();
        assert_eq!(
            result,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );

        assert_eq!(Solution::subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }
}
