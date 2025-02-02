impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut solutions = vec![];
        // String, Opening Parentheses Count, Closing Parentheses Count
        let mut stack = vec![("".to_string(), 0, 0)];

        while stack.len() > 0 {
            let (s, o_count, c_count) = stack.pop().unwrap();

            if o_count == n && c_count == n {
                solutions.push(s.to_string());
                continue;
            }

            if o_count < n {
                let mut s1 = s.clone();
                s1.push('(');
                stack.push((s1, o_count + 1, c_count));
            }

            if c_count < o_count {
                let mut s2 = s.clone();
                s2.push(')');
                stack.push((s2, o_count, c_count + 1));
            }
        }

        return solutions;
    }
}
