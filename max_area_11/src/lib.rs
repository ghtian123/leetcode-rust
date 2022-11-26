struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let mut result: i32 = 0;

        let mut left: i32 = 0;

        let mut right: i32 = (height.len() - 1) as i32;

        while left < right {
            let left_a = height[left as usize];
            let right_a = height[right as usize];

            let m_height = min(left_a, right_a);

            result = max(result, m_height * (right - left));

            if left_a < right_a {
                left += 1
            } else {
                right -= 1
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Solution;
        let height = vec![2, 3, 4, 5, 18, 17, 6];

        assert_eq!(Solution::max_area(height), 17);
    }
}
