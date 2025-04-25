// LeetCode/678 - Valid Parenthesis String

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut min, mut max) = (0 as i32, 0 as i32);
        for c in s.chars() {
            match c {
                '(' => {
                    (min, max) = (min + 1, max + 1);
                }
                ')' => {
                    (min, max) = (min - 1, max - 1);
                }
                _ => {
                    (min, max) = (min - 1, max + 1);
                }
            }
            if max < 0 {
                return false;
            }
            if min < 0 {
                min = 0;
            }
        }
        return min == 0;
    }

    pub fn check_valid_string_alt(s: String) -> bool {
        let n = s.len();
        let chars: Vec<_> = s.chars().collect();

        let mut free_a = 0;
        let mut free_b = n;

        let mut stack = vec![];
        for i in 0..n {
            let c = chars[i];
            while free_a < n && chars[free_a] != '*' {
                free_a += 1;
            }
            match c {
                '*' => {}
                '(' => {
                    stack.push(i);
                }
                ')' => {
                    if stack.len() > 0 {
                        stack.pop().unwrap();
                    } else if free_a < n && i > free_a {
                        free_a += 1;
                    } else {
                        return false;
                    }
                }
                _ => panic!("Invalid Char"),
            }
        }

        while stack.len() > 0 {
            while free_b > 0 && chars[free_b - 1] != '*' {
                free_b -= 1;
            }
            let i = stack.pop().unwrap();
            if free_b > free_a && i < free_b - 1 {
                free_b -= 1;
            } else {
                return false;
            }
        }

        return true;
    }
}
