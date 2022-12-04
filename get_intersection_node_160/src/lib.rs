use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn get_intersection_node(
        heada: Option<Rc<RefCell<ListNode>>>,
        headb: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        if heada.is_none() && headb.is_none() {
            return None;
        }

        let mut pa = heada.clone();
        let mut pb = headb.clone();

        while pa != pb {
            pa = if pa.is_none() {
                headb.clone()
            } else {
                pa.unwrap().borrow().next.clone()
            };

            pb = if pb.is_none() {
                headb.clone()
            } else {
                pb.unwrap().borrow().next.clone()
            }
        }

        pa
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
