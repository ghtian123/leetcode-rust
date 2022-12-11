use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
pub struct Node {
    key: i32,
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key: key,
            value: val,
            next: None,
            prev: None,
        }
    }
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_node_back(&mut self, node: Option<Rc<RefCell<Node>>>) {
        if let Some(t) = self.tail.take() {
            if let Some(n) = &node {
                t.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(Rc::downgrade(&t));
            }
        } else {
            self.head = node.clone();
        }

        self.tail = node;
    }

    fn remove_node(&mut self, node: Option<Rc<RefCell<Node>>>) {
        if let Some(mut n) = node {
            let mut prev = n.borrow_mut().prev.take();
            let mut next = n.borrow_mut().next.take();

            if let Some(p) = &mut prev {
                p.upgrade().map(|x| x.borrow_mut().next = next.clone());
            } else {
                self.head = next.clone();
            }

            if let Some(n) = &next {
                n.borrow_mut().prev = prev;
            } else {
                let p = prev.map(|x| x.upgrade());
                self.tail = if p.is_none() { None } else { p.unwrap() }
            }
        }
    }

    fn pop_front_node(&mut self) -> Option<Rc<RefCell<Node>>> {
        match self.head.take() {
            Some(head) => {
                if let Some(n) = head.borrow_mut().next.take() {
                    n.borrow_mut().prev = None;
                    self.head = Some(n);
                } else {
                    self.head = None;
                    self.tail = None;
                }

                Some(head)
            }
            None => None,
        }
    }
}

struct LRUCache {
    cap: usize,
    used: usize,
    data: HashMap<i32, Rc<RefCell<Node>>>,
    list: LinkedList,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            used: 0,
            data: HashMap::new(),
            list: LinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.data.get(&key) {
            let val = node.borrow().value;

            self.list.remove_node(Some(node.clone()));

            self.list.push_node_back(Some(node.clone()));

            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.data.get_mut(&key) {
            node.borrow_mut().key = key;
            node.borrow_mut().value = value;

            self.list.remove_node(Some(node.clone()));

            self.list.push_node_back(Some(node.clone()))
        } else {
            if self.used >= self.cap {
                if let Some(node) = self.list.pop_front_node() {
                    self.data.remove(&node.borrow().key);
                    self.used -= 1;
                }
            }

            let new_node = Rc::new(RefCell::new(Node::new(key, value)));
            self.data.insert(key, new_node.clone());

            self.list.push_node_back(Some(new_node));
            self.used += 1;
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
