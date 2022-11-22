struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;

        let sl = s.len();
        let tl = t.len();

        if sl == 0 || tl == 0 || sl < tl {
            return "".to_string();
        }

        let sb = s.as_bytes();
        let tb = s.as_bytes();

        let mut tFreq = HashMap::new();

        for i in tb {
            let count = tFreq.entry(*i).or_insert(0);
            *count += 1;
        }

        let mut instance = 0;
        let mut min_len = sl + 1;

        let mut begin = 0;

        let mut left = 0;
        let mut right = 0;

        for right in 0..sl {
            if !tFreq.contains_key(&sb[right]) {
                continue;
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
