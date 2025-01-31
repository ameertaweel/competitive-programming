impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 1 {
            return true;
        }

        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            let ci = s.bytes().nth(i).unwrap();
            if !ci.is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            let cj = s.bytes().nth(j).unwrap();
            if !cj.is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if ci.to_ascii_lowercase() != cj.to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1;
        }

        return true;
    }
}
