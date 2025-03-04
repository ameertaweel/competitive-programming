// LeetCode/211 - Design Add and Search Words Data Structure

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
#[derive(Clone)]
struct WordDictionary {
    terminal: bool,
    next: Vec<Option<Box<WordDictionary>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        return WordDictionary {
            terminal: false,
            next: vec![Option::None; 26],
        };
    }

    fn add_word(&mut self, word: String) {
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
        return self.search_helper(&word.chars().collect::<Vec<_>>());
    }

    fn search_helper(&self, txt: &[char]) -> bool {
        if txt.len() == 0 {
            return self.terminal;
        }
        if txt[0] != '.' {
            let char_code = (txt[0] as usize) - ('a' as usize);
            match &self.next[char_code] {
                Some(next) => return next.search_helper(&txt[1..]),
                None => return false,
            }
        }
        for i in 0..26 {
            let res = match &self.next[i] {
                Some(next) => next.search_helper(&txt[1..]),
                None => false,
            };
            if res {
                return true;
            }
        }
        return false;
    }
}
