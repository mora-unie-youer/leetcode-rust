/**
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        *nums = nums
            .iter()
            .enumerate()
            .filter_map(|(i, &n)| {
                if i > 1 && nums[i - 2] == n {
                    None
                } else {
                    Some(n)
                }
            })
            .collect();

        nums.len() as i32
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        Solution::remove_duplicates(&mut nums);
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);

        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        Solution::remove_duplicates(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
