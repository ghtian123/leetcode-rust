use std::collections::HashMap;
use std::ptr::drop_in_place;
use std::{marker::PhantomData, ptr::NonNull};

//ref : https://zhuanlan.zhihu.com/p/466409120
pub struct Node<T: Default> {
    ele: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Default> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            ele: elem,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T: Default> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    // 幽灵数据，本质上是一个标记。因为LinkedList使用了NonNull，类似裸指针，
    // 所以LinkedList和T之间的关系就有点模糊不清了。
    // 比如LinkedList析构的时候是否需要析构T，
    // 如果把LinkedList使用默认的析构函数，
    // 那么T肯定没有被析构，存在内存泄露。
    // 所以使用LinkedList的人就会比较迷惑，所以需要加个标记，
    // 标记LinkedList拥有T，即LinkedList析构，T也就析构了，
    // 同理T的生命周期不可能超过LinkedList，这里的不超过指的是生命周期的结束点
    marker: PhantomData<T>,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }
    pub fn push_back(&mut self, ele: T) {
        // 这里使用Box::new创建一个堆对象，
        // 然后通过Box::leak主动将Box对象泄露出去，
        // 因为Box类似unique_ptr，离开作用域就析构了
        // 所以Box::leak(Box::new) 就等效于C++中的new，如果不主动释放则存在内存泄露
        let node = Box::leak(Box::new(Node {
            ele,
            prev: self.tail,
            next: None,
        }))
        .into();
        // 这里和之前的safe代码很像了。
        // 如果函数有返回值，推荐使用Option::map方法。
        // 如果函数没有返回值，推荐使用match匹配Option。
        // 这里为什么没有使用 match self.tail.take呢？
        // 是不是self.tail所有权被移走了呢？
        // 其实这里是因为NonNull具有Copy语义，复制了。
        // 如果T是Copy语义，Option<T>也具备Copy语义
        match self.tail {
            Some(mut tail) => unsafe {
                tail.as_mut().next = Some(node);
                self.tail = Some(node);
            },
            None => {
                self.head = Some(node);
                self.tail = Some(node);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // 如push_back函数说明一样，这里采用Option::map方法
        // 主意NonNull的as_mut方法的签名是pub unsafe fn as_mut<'a>(&mut self) -> &'a mut T
        // 所以需要NonNull是可变的，所以在map的闭包中的参数是mut
        self.tail.take().map(|mut tail| unsafe {
            match tail.as_mut().prev {
                Some(mut prev) => {
                    prev.as_mut().next = None;
                    tail.as_mut().prev = None;
                    self.tail = Some(prev);
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            // 这里特别要主意一下，因为是pop_back，
            // 所以需要主动释放对应Node的内存
            // 这里就类似让unique_ptr重新接管裸指针，
            // 离开作用域自动析构
            let mut node = Box::from_raw(tail.as_ptr());
            // 这里把node的ele成员所有权转移走了
            // 为什么这里可以转移某个变量的成员呢？
            // 之前写Safe Rust的时候，self.tail的所有权为什么无法转移呢？
            // 这是因为node已经没有地方再使用了，而且node没有自定义析构函数。
            // 可以认为ele成员没有任何地方再使用了，所以可以安全被转移走了。

            std::mem::replace(&mut node.ele, T::default())
            // node.ele
        })
    }

    pub fn peek_back(&self) -> Option<&T> {
        // 这里的peek_back就比较简单了，而且返回值更加自然Option<&T>，
        // 而Safe Code只能返回Option<Ref<T>>，原因见Safe Code代码说明
        self.tail.map(|node| unsafe { &node.as_ref().ele })
    }

    pub fn remove_node_from_list(&mut self, node: Option<NonNull<Node<T>>>) {
        if let Some(mut n) = node {
            unsafe {
                let mut prev = (*n.as_ptr()).prev.take();
                let mut next = (*n.as_ptr()).next.take();
                if let Some(p) = &mut prev {
                    (*p.as_ptr()).next = next;
                } else {
                    self.head = next;
                }

                if let Some(n) = &mut next {
                    n.as_mut().prev = prev;
                } else {
                    self.tail = prev;
                }
            }
        }
    }

    pub fn push_node_back(&mut self, mut node: Option<NonNull<Node<T>>>) {
        unsafe {
            if let Some(mut t) = self.tail.take() {
                if let Some(n) = &mut node {
                    (*t.as_ptr()).next = Some(*n);
                    (*n.as_ptr()).prev = Some(t);
                }
            } else {
                self.head = node;
            }

            self.tail = node;
        }
    }

    pub fn pop_front_node(&mut self) -> Option<NonNull<Node<T>>> {
        unsafe {
            match self.head.take() {
                Some(mut head) => {
                    if let Some(mut n) = (*head.as_ptr()).next.take() {
                        (*n.as_ptr()).prev = None;
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
}

impl<T: Default> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

// NonNull实现了Copy语义
// 可以通过Box::leak(Box::new).into生成NonNull，类似C++中的new
// NonNull的as_mut方法需要NonNull自身是可变的
// NonNull需要手动释放内存，可以通过Box::from_raw(xx.as_ptr())，让智能指针重新接管裸指针
struct LRUCacheI32<T: Default> {
    cap: usize,
    used: usize,
    data: HashMap<i32, NonNull<Node<T>>>,
    list: LinkedList<T>,
}

impl LRUCacheI32<(i32, i32)> {
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
            self.list.remove_node_from_list(Some(*node));

            self.list.push_node_back(Some(*node));

            unsafe {
                let val = (*node.as_ptr()).ele.1;

                val
            }
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        unsafe {
            if let Some(node) = self.data.get_mut(&key) {
                node.as_mut().ele = (key, value);

                self.list.remove_node_from_list(Some(*node));

                self.list.push_node_back(Some(*node))
            } else {
                if self.used >= self.cap {
                    if let Some(node) = self.list.pop_front_node() {
                        self.data.remove(&node.as_ref().ele.0);

                        drop_in_place(node.as_ptr());
                        // let node = Box::from_raw();
                        // // //释放内存
                        // drop(node);
                        self.used -= 1;
                    }
                }

                let new_node = Box::leak(Box::new(Node::new((key, value)))).into();

                self.data.insert(key, new_node);

                self.list.push_node_back(Some(new_node));
                self.used += 1;
            }
        }
    }
}

struct LRUCache {
    cache: LRUCacheI32<(i32, i32)>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cache: LRUCacheI32::new(capacity),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        self.cache.get(key)
    }

    fn put(&mut self, key: i32, value: i32) {
        self.cache.put(key, value)
    }
}

#[test]
fn test() {}
