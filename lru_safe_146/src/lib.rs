use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

//ref : https://zhuanlan.zhihu.com/p/466409120

pub struct Node<T> {
    ele: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(ele: T) -> Self {
        Self {
            ele,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, ele: T) {
        // 创建node，不需要设置为mut
        // 因为prev, next字段都由RefCell包裹，
        // 所以prev, next都是可变的
        let node = Rc::new(RefCell::new(Node::new(ele)));
        // 这里要特别主意一下
        // 因为match会导致所有权转移
        // tail是属于self的字段，rust不允许所有权被转移走
        // 这里使用Option的take方法，把内部值转移走，而self.tail变为None
        match self.tail.take() {
            Some(tail) => {
                // borror_mut是RefCell的方法，让内部的值变为可变
                // Rc没有实现Copy Trait，但是实现clone方法，不过需要手动调用一下该方法
                // 代码逻辑比较简单了: 如果tail存在则往后追加节点，并把节点链接起来
                tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(tail);
                self.tail = Some(node);
            }
            None => {
                // 如果self.tail是None表示第一次push，则更新一下self.head
                // 因为双向链表只有一个值，self.head和self.tail应该是一样的
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // take方法见push_back方法中的注解
        // 因为pop_back方法有返回值，采用Option::map的方式
        // 比较自然，如果self.tail是None就直接返回None
        self.tail.take().map(|node| {
            // 判断最后一个节点有没有prev节点
            // 如果有则断开，如果没有则把self.head和tail一起变成None
            match node.borrow_mut().prev.take() {
                Some(head) => {
                    head.borrow_mut().next = None;
                    self.tail = Some(head);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            // 这里比较关键
            // 我们来捋捋，node是Rc类型，
            // 表示智能指针，共享了所有权
            // 但是pop则表示把node从双向链表中删除，即所有权转移走
            // 我们又不知道有没有其他地方共享了所有权，所以使用Rc::try_unwrap
            // 这个try很关键，因为编译器不知道，所以需要运行时判断
            // 中间的ok函数表示把Result类型转成Option类型
            // into_inner是将RefCell<T>转成T，最终所有权被释放出来了
            Rc::try_unwrap(node).ok().unwrap().into_inner().ele
        })
    }

    // pub fn peek_back(&self) -> Option<&T> {
    //     self.tail.as_ref().map(|node| {
    //         // error[E0515]: cannot return value referencing temporary value
    //         // 这是因为node.borrow()返回了一个临时变量Ref<Node<T>>
    //         // 所以无法返回临时变量的引用（这个也是C++中常见的问题）
    //         &node.borrow().ele
    //     })
    // }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            // 由于node.borrow()返回的是Ref<Node<T>>
            // 如果peek_back直接返回Ref<Node<T>>，则把内部的细节Node类型
            // 暴露给用户，所以需要把内部细节屏蔽掉
            // 使用Ref::map可以把内部字段映射出来
            Ref::map(node.borrow(), |node| &node.ele)
        })
    }

    pub fn remove_node(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        if let Some(n) = node {
            let prev = n.borrow_mut().prev.take();
            let next = n.borrow_mut().next.take();

            if let Some(p) = &prev {
                p.borrow_mut().next = next.clone();
            } else {
                self.head = next.clone();
            }

            if let Some(n) = &next {
                n.borrow_mut().prev = prev;
            } else {
                self.tail = prev;
            }
        }
    }

    pub fn push_node_back(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        if let Some(t) = self.tail.take() {
            if let Some(n) = &node {
                t.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(t);
            }
        } else {
            self.head = node.clone();
        }

        self.tail = node;
    }

    pub fn pop_front_node(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
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

// 由于节点相互依赖，所以无法主动释放内存
// 需要自定义析构函数主动释放(pop_back)所有元素
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

struct LRUCache<T> {
    cap: usize,
    used: usize,
    data: HashMap<i32, Rc<RefCell<Node<T>>>>,
    list: LinkedList<T>,
}

impl LRUCache<(i32, i32)> {
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
            let val = node.borrow().ele.1;

            self.list.remove_node(Some(node.clone()));

            self.list.push_node_back(Some(node.clone()));

            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.data.get_mut(&key) {
            node.borrow_mut().ele = (key, value);

            self.list.remove_node(Some(node.clone()));

            self.list.push_node_back(Some(node.clone()))
        } else {
            if self.used >= self.cap {
                if let Some(node) = self.list.pop_front_node() {
                    self.data.remove(&node.borrow().ele.0);
                    self.used -= 1;
                }
            }

            let new_node = Rc::new(RefCell::new(Node::new((key, value))));
            self.data.insert(key, new_node.clone());

            self.list.push_node_back(Some(new_node));
            self.used += 1;
        }
    }
}
