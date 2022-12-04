struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;

        while fast < nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1
            }
            fast += 1
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Solution;
        let mut v = vec![0, 0, 1];
        Solution::move_zeroes(&mut v);

        assert_eq!(v, vec![1, 0, 0]);
    }
}
