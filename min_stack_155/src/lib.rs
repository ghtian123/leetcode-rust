struct MinStack {
    min_stack: Vec<i32>,
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        let mut mins = Self {
            min_stack: Vec::new(),
            stack: Vec::new(),
        };
        mins.min_stack.push(i32::MAX);
        mins
    }

    //将元素val推入堆栈
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.min_stack
            .push((*self.min_stack.last().unwrap()).min(val));
    }

    //删除堆栈顶部的元素
    fn pop(&mut self) {
        if !self.stack.is_empty() {
            self.stack.pop();
            self.min_stack.pop();
        }
    }

    //获取堆栈顶部的元素
    fn top(&mut self) -> i32 {
        if !self.stack.is_empty() {
            return *self.stack.last().unwrap();
        }
        0
    }

    //获取堆栈中的最小元素
    fn get_min(&self) -> i32 {
        if !self.stack.is_empty() {
            return *self.min_stack.last().unwrap();
        }
        0
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
