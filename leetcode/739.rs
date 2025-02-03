impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut solutions = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            let t = temperatures[i];

            while stack.len() > 0 {
                let (i_prev, t_prev) = stack.pop().unwrap();
                if t <= t_prev {
                    stack.push((i_prev, t_prev));
                    break;
                }
                solutions[i_prev] = (i - i_prev) as i32;
            }

            stack.push((i, t));
        }

        return solutions;
    }
}
