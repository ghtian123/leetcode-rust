use std::ops::Index;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // nums.into_iter().reduce(|a,b| a^b).unwrap();
        nums.into_iter().fold(0, |acc, x| acc ^ x)
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
