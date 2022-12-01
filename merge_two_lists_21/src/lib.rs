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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);

        let mut curr = &mut head;

        while let (Some(l1), Some(l2)) = (list1.as_mut(), list2.as_mut()) {
            if l1.val < l2.val {
                let next = l1.next.take();
                curr.next = list1;
                list1 = next;
            } else {
                let next = l2.next.take();
                curr.next = list2;
                list2 = next;
            }

            curr = curr.next.as_mut().unwrap();
        }

        curr.next = if list1.is_none() { list2 } else { list1 };

        head.next
    }

    //所有权转移
    pub fn merge_two_lists1(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);

        let mut curr = &mut head;

        if list1.is_none() && list2.is_some() {
            curr.next = list2;
            return head.next;
        }

        if list1.is_some() && list2.is_none() {
            curr.next = list1;
            return head.next;
        }

        while let (Some(mut l1), Some(mut l2)) = (list1, list2) {
            if l1.val < l2.val {
                let next = l1.next.take();
                curr.next = Some(l1);

                list2 = Some(l2);
                list1 = next;
            } else {
                let next = l2.next.take();
                curr.next = Some(l2);

                list1 = Some(l1);
                list2 = next;
            }

            curr = curr.next.as_mut().unwrap();

            if list1.is_none() && list2.is_some() {
                curr.next = list2;
                break;
            }

            if list1.is_some() && list2.is_none() {
                curr.next = list1;
                break;
            }
        }

        head.next
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
