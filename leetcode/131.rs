impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars: Vec<_> = s.chars().collect();
        let mut answers = vec![];
        let mut partial = vec![];
        Self::parition_helper(&s, &chars, &mut answers, &mut partial, 0);
        return answers;
    }

    pub fn parition_helper(
        s: &String,
        chars: &Vec<char>,
        answers: &mut Vec<Vec<String>>,
        partial: &mut Vec<String>,
        len: usize,
    ) {
        if len == s.len() {
            answers.push(partial.clone());
            return;
        }
        for i in len..s.len() {
            if !Self::is_palindrome(chars, len, i) {
                continue;
            }
            partial.push(s[len..(i + 1)].to_string());
            Self::parition_helper(s, chars, answers, partial, i + 1);
            partial.pop().unwrap();
        }
    }

    pub fn is_palindrome(s: &Vec<char>, beg: usize, end: usize) -> bool {
        let mut beg = beg;
        let mut end = end;
        while beg < end {
            if s[beg] != s[end] {
                return false;
            }
            beg += 1;
            end -= 1;
        }
        return true;
    }
}
