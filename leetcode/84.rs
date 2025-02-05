impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = heights[0];
        let mut h_prev = heights[0];
        // Width, Height
        let mut stack = vec![(1, heights[0])];

        for &h in &heights[1..] {
            if h < h_prev {
                while stack.len() >= 2 && stack[stack.len() - 2].1 >= h {
                    let (w2, _h2) = stack.pop().unwrap();
                    let (w1, h1) = stack.pop().unwrap();
                    let w = w1 + w2;
                    let h = h1;
                    stack.push((w, h));
                    let area = w * h;
                    if area > max_area {
                        max_area = area;
                    }
                }
            }

            let w = if h <= h_prev {
                let (w_prev, _h_prev) = stack.pop().unwrap();
                w_prev + 1
            } else {
                1
            };
            stack.push((w, h));
            let area = w * h;
            if area > max_area {
                max_area = area;
            }
            h_prev = h;
        }

        while stack.len() >= 2 {
            let (w2, _h2) = stack.pop().unwrap();
            let (w1, h1) = stack.pop().unwrap();
            let w = w1 + w2;
            let h = h1;
            stack.push((w, h));
            let area = w * h;
            if area > max_area {
                max_area = area;
            }
        }

        return max_area;
    }
}
