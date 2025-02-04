impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut v: Vec<_> = position.into_iter().zip(speed.into_iter()).collect();
        v.sort_by(|a, b| b.0.cmp(&a.0));
        let v = v;

        let mut stack = vec![];
        for (p, s) in v {
            let d = target - p;
            let t = d as f32 / s as f32;
            if stack.len() == 0 {
                stack.push(t);
                continue;
            }
            let t_prev = stack[stack.len() - 1];
            if t > t_prev {
                stack.push(t);
            }
        }

        return stack.len() as i32;
    }
}
