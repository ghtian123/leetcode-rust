use std::ptr::NonNull;

#[derive(PartialEq)]
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

    pub fn has_cycle1(head: Option<NonNull<ListNodeNonNull>>) -> bool {
        unsafe {
            if head.is_none() || (*head.unwrap().as_ptr()).next.is_none() {
                return false;
            }

            let mut slow = head;
            let mut fast = (*head.unwrap().as_ptr()).next;

            while slow != fast {
                if fast.is_none() || (*fast.unwrap().as_ptr()).next.is_none() {
                    return false;
                }
                slow = (*slow.unwrap().as_ptr()).next;
                fast = (*fast.unwrap().as_ptr()).next;
                fast = (*fast.unwrap().as_ptr()).next;
            }

            return true;
        }
    }

    pub fn has_cycle2(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return false;
        }

        let mut slow = head.as_ref();
        let mut fast = head.as_ref().unwrap().next.as_ref();

        while slow != fast {
            if fast.is_none() || fast.unwrap().next.is_none() {
                return false;
            }
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref();
        }

        return true;
    }
}

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
// func hasCycle(head *ListNode) bool {

//     if head == nil || head.Next == nil{
//         return false
//     }

//     slow := head
//     fast := head.Next

//     for slow != fast{
//         if fast ==nil || fast.Next==nil {
//             return false
//         }
//         slow = slow.Next
//         fast = fast.Next.Next

//     }

//     return true
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
