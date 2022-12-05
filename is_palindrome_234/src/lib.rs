// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut stack = Vec::new();

        while head.is_some() {
            let mut n = head.as_mut().unwrap().next.take();
            stack.push(head);
            head = n;
        }

        for i in 0..stack.len() / 2 {
            if stack[i] != stack[stack.len() - 1 - i] {
                return false;
            }
        }

        return true;
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
