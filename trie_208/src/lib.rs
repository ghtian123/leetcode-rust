#[derive(Default, Clone)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;

        for i in word.bytes().into_iter().map(|x| (x - b'a') as usize) {
            node = node.children[i].get_or_insert_with(|| Box::new(Self::default()));
        }
        node.is_end = true;
    }

    fn search(&mut self, word: String) -> bool {
        let node = self.search_prefix(word);
        node.is_some() && node.unwrap().is_end
    }

    fn starts_with(&mut self, prefix: String) -> bool {
        self.search_prefix(prefix).is_some()
    }

    fn search_prefix(&mut self, prefix: String) -> Option<Box<Trie>> {
        let mut node = self;

        for i in prefix.bytes().into_iter().map(|x| (x - b'a') as usize) {
            if node.children[i].is_none() {
                return None;
            }
            node = node.children[i].as_mut().unwrap();
        }

        return Some(Box::new(node.clone()));
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
