use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let parens = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);

        let mut stack = vec![];

        for c in s.chars() {
            if parens.contains_key(&c) {
                stack.push(c);
                continue;
            }

            if stack.len() == 0 || c != parens[&stack.pop().unwrap()] {
                return false;
            }
        }

        return stack.len() == 0;
    }
}
