struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let count = nums.len() / 2;
        let mut m = HashMap::new();

        nums.into_iter().for_each(|x| {
            let mut c = m.entry(x).or_insert(0);
            *c += 1
        });

        m.into_iter()
            .filter(|(k, v)| *v >= count)
            .max_by_key(|(k, v)| *v)
            .unwrap()
            .0
    }

    pub fn majority_element1(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
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
