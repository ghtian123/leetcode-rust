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

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(mut r1), Some(mut r2)) => {
                unsafe {
                    (*r1.as_ptr()).val = r1.borrow().val + r2.borrow().val;
                    (*r1.as_ptr()).left = Self::merge_trees(
                        r1.as_ref().borrow_mut().left.take(),
                        r2.as_ref().borrow_mut().left.take(),
                    );
                    (*r1.as_ptr()).right = Self::merge_trees(
                        r1.as_ref().borrow_mut().right.take(),
                        r2.as_ref().borrow_mut().right.take(),
                    );
                }
                return Some(r1);
            }
            (Some(r1), None) => {
                return Some(r1);
            }
            (None, Some(r2)) => {
                return Some(r2);
            }
            (None, None) => {
                return None;
            }
        }
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
