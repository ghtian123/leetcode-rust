struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let mut min_p = i32::MAX;
        let mut max_p = 0;
        prices
            .into_iter()
            .map(|x| {
                max_p = max(max_p, x - min_p);
                min_p = min(min_p, x);
                max_p
            })
            .max()
            .unwrap()
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
