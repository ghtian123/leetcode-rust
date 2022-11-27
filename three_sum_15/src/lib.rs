struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let length = nums.len();

        if length < 3 {
            return result;
        }

        nums.sort();

        let mut left = 0;
        let mut right = 0;

        for i in 0..length {
            if nums[i] > 0 {
                return result;
            }

            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            left = i + 1;
            right = length - 1;

            while left < right {
                match (nums[left] + nums[right] + nums[i]).cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[left], nums[right], nums[i]]);
                        while left < right && nums[left] == nums[left + 1] {
                            left = left + 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right = right - 1;
                        }
                        left = left + 1;
                        right = right - 1;
                    }
                    std::cmp::Ordering::Greater => {
                        right = right - 1;
                    }
                    std::cmp::Ordering::Less => {
                        left = left + 1;
                    }
                }
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

        let nums = vec![-1, 0, 1, 2, -1, -4];

        assert_eq!(
            Solution::three_sum(nums),
            vec![vec![-1, 2, -1], vec![0, 1, -1]]
        );
    }
}
