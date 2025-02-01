/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
struct MinStack {
    vals: Vec<i32>,
    mins: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            vals: vec![],
            mins: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.vals.push(val);

        let prev_min = if self.mins.len() > 0 {
            self.mins[self.mins.len() - 1]
        } else {
            val
        };
        let curr_min = if val < prev_min { val } else { prev_min };
        self.mins.push(curr_min);
    }

    fn pop(&mut self) {
        self.vals.pop();
        self.mins.pop();
    }

    fn top(&self) -> i32 {
        self.vals[self.vals.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.mins[self.mins.len() - 1]
    }
}
