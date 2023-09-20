/**
 * 70. Climbing Stairs
 *
 * You are climbing a staircase. It takes n steps to reach the top.
 *
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 *
 *
 * Example 1:
 *   Input: n = 2
 *   Output: 2
 *   Explanation: There are two ways to climb to the top.
 *   1. 1 step + 1 step
 *   2. 2 steps
 *
 * Example 2:
 *   Input: n = 3
 *   Output: 3
 *   Explanation: There are three ways to climb to the top.
 *   1. 1 step + 1 step + 1 step
 *   2. 1 step + 2 steps
 *   3. 2 steps + 1 step
 *
 *
 * Constraints:
 *   1 <= n <= 45
 */
pub struct Solution;
fn main() {}

// SUBMISSION CODE START

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;

        let mut dp = vec![0; n + 2];
        dp[n] = 1;
        for i in (0..n).rev() {
            dp[i] = dp[i + 1] + dp[i + 2];
        }

        dp[0]
    }
}

// SUBMISSION CODE END

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
