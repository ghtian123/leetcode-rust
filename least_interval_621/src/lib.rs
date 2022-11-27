struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;

        let mut mp = HashMap::new();
        let mut max_num = 0;

        //for in 会调用into_iter
        for task in tasks.iter() {
            let entry = mp.entry(task).or_insert(0);
            *entry += 1;
            max_num = max_num.max(*entry);
        }

        let cnt = mp.iter().filter(|(_, t)| **t == max_num).count() as i32;

        max((max_num - 1) * (n + 1) + cnt, tasks.len() as i32)
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
