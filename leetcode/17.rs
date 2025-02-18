impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 { return vec![]; }
        let digits: Vec<_> = digits.chars().collect();
        let mut answers = vec![];
        let mut partial = String::new();
        Self::letter_combinations_helper(&digits, &mut answers, &mut partial);
        return answers;
    }

    pub fn letter_combinations_helper(digits: &Vec<char>, answers: &mut Vec<String>, partial: &mut String) {
        if partial.len() == digits.len() {
            answers.push(partial.clone());
            return;
        }

        let chars = match digits[partial.len()] {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
             _  => panic!("Invalid Digit"),
        };

        for &c in &chars {
            partial.push(c);
            Self::letter_combinations_helper(digits, answers, partial);
            partial.pop().unwrap();
        }
    }
}
