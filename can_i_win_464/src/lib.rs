struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total == 0 {
            return true;
        }
        if max_choosable_integer * (1 + max_choosable_integer) / 2 < desired_total {
            return false;
        }
        fn dfs(curr: i32, choosable: i32, total: i32, dp: &mut Vec<i32>) -> bool {
            if total <= 0 {
                return false;
            }
            if dp[curr as usize] != 0 {
                return dp[curr as usize] == 1;
            }
            for i in 0..choosable {
                if (curr & (1 << i)) == 0 && !dfs(curr | (1 << i), choosable, total - i - 1, dp) {
                    dp[curr as usize] = 1;
                    return true;
                }
            }
            dp[curr as usize] = -1;
            false
        }
        dfs(
            0,
            max_choosable_integer,
            desired_total,
            &mut vec![0; 1 << max_choosable_integer],
        )
    }
}
