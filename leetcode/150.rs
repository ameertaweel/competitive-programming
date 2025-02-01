impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for t in tokens {
            if t != "+" && t != "-" && t != "*" && t != "/" {
                let n: i32 = t.parse().unwrap();
                stack.push(n);
                continue;
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let c = if t == "+" {
                a + b
            } else if t == "-" {
                a - b
            } else if t == "*" {
                a * b
            } else {
                a / b
            };
            stack.push(c);
        }

        return stack.pop().unwrap();
    }
}
