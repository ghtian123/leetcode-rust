// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(root.clone(), root.clone())
    }

    pub fn check(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = vec![left, right];

        while !stack.is_empty() {
            let left = stack.remove(0);
            let right = stack.remove(0);

            if left.is_none() && right.is_none() {
                continue;
            }
            if (left.is_none() || right.is_none())
                || (left.as_ref().unwrap().borrow().val != right.as_ref().unwrap().borrow().val)
            {
                return false;
            }

            stack.push(left.as_ref().unwrap().borrow_mut().left.take());
            stack.push(right.as_ref().unwrap().borrow_mut().right.take());

            stack.push(left.as_ref().unwrap().borrow_mut().right.take());

            stack.push(right.as_ref().unwrap().borrow_mut().left.take());
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
