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

    pub fn longest_palindrome_window(s: String) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut window = s.len();
        let mut head = 0;
        while head != sv.len() {
            if head + window > sv.len() {
                window -= 1;
                head = 0;
                continue;
            }
            if Self::is_palindrome(&sv[head..head + window]) {
                return sv[head..head + window].iter().collect::<String>();
            }
            head += 1;
        }

        return "".to_string();
    }

    fn is_palindrome(v: &[char]) -> bool {
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        true
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
