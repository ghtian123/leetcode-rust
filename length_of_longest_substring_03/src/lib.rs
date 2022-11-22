struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;

        let bs = s.as_bytes();
        let len = s.len();

        if len == 0 {
            return 0;
        }
        let mut lookup = HashMap::new();

        let mut left = 0;
        let mut max_len = 0;

        for i in 0..len {
            if let Some(index) = lookup.get(&bs[i]) {
                left = max(left, *index + 1)
            }
            lookup.insert(bs[i], i);
            max_len = max(max_len, i - left + 1)
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Solution;
        let s = String::from("abcabcbb");

        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
