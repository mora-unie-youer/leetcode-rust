/**
 * 4. Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 *
 * The overall run time complexity should be O(log (m+n)).
 *
 * Example 1:
 *   Input: nums1 = [1,3], nums2 = [2]
 *   Output: 2.00000
 *   Explanation: merged array = [1,2,3] and median is 2.
 *
 * Example 2:
 *   Input: nums1 = [1,2], nums2 = [3,4]
 *   Output: 2.50000
 *   Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 * Constraints:
 *   nums1.length == m
 *   nums2.length == n
 *   0 <= m <= 1000
 *   0 <= n <= 1000
 *   1 <= m + n <= 2000
 *   -106 <= nums1[i], nums2[i] <= 106
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

use std::{cmp::Ordering, ops::Range};

impl Solution {
    fn binary_search(range: Range<usize>, fnx: impl Fn(usize) -> Ordering) -> Result<usize, usize> {
        let Range { mut start, mut end } = range;

        while end > start {
            let next = (start + end) / 2;
            match fnx(next) {
                Ordering::Less => start = next + 1,
                Ordering::Greater => end = next,
                Ordering::Equal => return Ok(next),
            }
        }

        Err(end)
    }

    fn option_min(v1: Option<i32>, v2: Option<i32>) -> Option<i32> {
        match (v1, v2) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), _) => Some(a),
            (_, Some(b)) => Some(b),
            _ => None,
        }
    }

    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let total_length = nums1.len() + nums2.len();
        let mid = (total_length - 1) / 2;

        let partition = Self::binary_search(0..nums1.len(), |x| {
            if mid - x > 0 && nums2[mid - x - 1] > nums1[x] {
                Ordering::Less
            } else if nums2[mid - x] < nums1[x] {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        match (partition, total_length % 2 == 1) {
            (Ok(x), true) => nums1[x] as f64,
            (Err(x), true) => nums2[mid - x] as f64,
            (Ok(x), false) => {
                let second_value =
                    Self::option_min(nums1.get(x + 1).copied(), nums2.get(mid - x).copied());
                (nums1[x] + second_value.unwrap()) as f64 / 2.0
            }
            (Err(x), false) => {
                let second_value =
                    Self::option_min(nums1.get(x).copied(), nums2.get(mid - x + 1).copied());
                (nums2[mid - x] + second_value.unwrap()) as f64 / 2.0
            }
        }
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
