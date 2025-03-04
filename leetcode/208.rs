// LeetCode/208 - Implement Trie (Prefix Tree)

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
#[derive(Clone)]
struct Trie {
    terminal: bool,
    next: Vec<Option<Box<Trie>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        return Trie {
            terminal: false,
            next: vec![Option::None; 26],
        };
    }

    fn insert(&mut self, word: String) {
        let mut ptr = self;
        for c in word.bytes() {
            let char_code = (c as usize) - ('a' as usize);
            if ptr.next[char_code].is_none() {
                ptr.next[char_code] = Option::Some(Box::new(Self::new()));
            }
            ptr = ptr.next[char_code].as_mut().unwrap();
        }
        ptr.terminal = true;
    }

    fn search(&self, word: String) -> bool {
        let mut ptr = self;
        for c in word.bytes() {
            let char_code = (c as usize) - ('a' as usize);
            match &ptr.next[char_code] {
                Some(next) => ptr = next,
                None => return false,
            }
        }
        return ptr.terminal;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut ptr = self;
        for c in prefix.bytes() {
            let char_code = (c as usize) - ('a' as usize);
            match &ptr.next[char_code] {
                Some(next) => ptr = next,
                None => return false,
            }
        }
        return true;
    }
}
