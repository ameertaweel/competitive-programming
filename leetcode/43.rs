// LeetCode/43 - Multiply Strings

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let n1 = num1
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();
        let n2 = num2
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>();

        let mut ans = vec![];
        let mut carry = 0;

        for i2 in 0..n2.len() {
            for i1 in 0..n1.len() {
                let i = i1 + i2;
                if i == ans.len() {
                    ans.push(0);
                }
                let value = n1[i1] * n2[i2] + carry + ans[i];
                let d = value % 10;
                let c = value / 10;
                ans[i] = d;
                carry = c;
            }
            let mut i = n1.len() + i2;
            while carry > 0 {
                if i == ans.len() {
                    ans.push(0);
                }
                ans[i] = carry % 10;
                carry /= 10;
                i += 1;
            }
        }

        while ans.len() > 1 && *ans.last().unwrap() == 0 {
            ans.pop();
        }

        // Reverse the vector in-place
        ans.reverse();

        // Convert each digit to a string, then collect into one string
        return ans.iter().map(|d| d.to_string()).collect();
    }
}
