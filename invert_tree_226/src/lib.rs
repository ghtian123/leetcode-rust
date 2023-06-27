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

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            let left = root.as_ref().unwrap().as_ref().borrow_mut().left.take();
            let right = root.as_ref().unwrap().as_ref().borrow_mut().right.take();
            root.as_ref().unwrap().as_ref().borrow_mut().left = Self::invert_tree(right);
            root.as_ref().unwrap().as_ref().borrow_mut().right = Self::invert_tree(left);
        }

        root
    }
}

// class Solution:
//     def invertTree(self, root: TreeNode) -> TreeNode:
//         if root:
//             root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
//         return root

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
