use std::ptr::NonNull;


pub struct ListNodeNonNull {
    pub val: i32,
    pub next: Option<NonNull<ListNodeNonNull>>,
}

impl ListNodeNonNull {
    #[inline]
    fn new(val: i32) -> Self {
        ListNodeNonNull { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<NonNull<ListNodeNonNull>>) -> bool {
        unsafe {
            let mut slow = head;
            let mut fast = head;

            while slow.is_some() && fast.is_some() && (*fast.unwrap().as_ptr()).next.is_some() {
                slow = (*slow.unwrap().as_ptr()).next;
                fast = (*fast.unwrap().as_ptr()).next;
                fast = (*fast.unwrap().as_ptr()).next;
                if slow == fast {
                    return true;
                }
            }
            false
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
