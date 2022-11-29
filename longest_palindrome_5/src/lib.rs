struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let sl = s.len();

        if sl < 2 {
            return s;
        }

        let mut dp: Vec<Vec<bool>> = vec![vec![true; sl]; sl];
        let mut sv = s.chars().collect::<Vec<char>>();
        let mut start = 0;
        let mut maxLen = 1;

        for j in 1..sl {
            for i in 0..j {
                if sv[i] != sv[j] {
                    dp[i][j] = false
                } else {
                    dp[i][j] = dp[i + 1][j - 1]
                }
                if dp[i][j] && j - i + 1 > maxLen {
                    maxLen = j - i + 1;
                    start = i;
                }
            }
        }

        return s[start..start + maxLen].to_owned();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Solution;

        let s = String::from("babad");

        assert_eq!(Solution::longest_palindrome(s), "bab");
    }
}
