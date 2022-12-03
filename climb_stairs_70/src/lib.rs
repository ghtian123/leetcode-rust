struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = Vec::new();
        dp.insert(0, 1);
        dp.insert(1, 1);

        for i in 2..n + 1 {
            dp.insert(
                i as usize,
                dp.get((i - 1) as usize).unwrap() + dp.get((i - 2) as usize).unwrap(),
            )
        }
        dp[n as usize]
    }

    pub fn climb_stairs1(n: i32) -> i32 {
        if n < 2 {
            return 1;
        } else {
            return Self::climb_stairs1(n - 1) + Self::climb_stairs1(n - 2);
        }
    }

    pub fn climb_stairs2(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        (0..n).for_each(|_| {
            b = a + b;
            a = b - a;
        });
        a
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
