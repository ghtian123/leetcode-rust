struct Solution;

//最长和最短的交集
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let max_s = strs.iter().max().unwrap().as_bytes();

        let min_s = strs.iter().min().unwrap().as_bytes();

        let mut index = 0;

        while index < min_s.len() && max_s[index] == min_s[index] {
            index += 1
        }

        String::from_utf8(max_s[..index].to_owned()).unwrap()
    }

    pub fn longest_common_prefix1(strs: Vec<String>) -> String {
        strs.iter()
            .max()
            .unwrap()
            .chars()
            .zip(strs.iter().min().unwrap().chars())
            .take_while(|x| x.0 == x.1)
            .map(|x| x.0)
            .collect()
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
