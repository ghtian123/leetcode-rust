#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for (k, v) in nums.into_iter().enumerate() {
            match map.get(&(target - v)) {
                Some(i) => return vec![*i as i32, k as i32],
                None => {
                    map.insert(v, k);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let nums = vec![11, 15, 2, 7];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![2, 3]);
    }
}
