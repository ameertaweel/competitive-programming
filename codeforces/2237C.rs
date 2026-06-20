// - Let's say we have three consecutive numbers in reverse order:
//   x > y > z
// - We can swap in two ways:
//   - Swap from right:
//     - x, y    , z
//     - x, z    , y + z
//     - z, x + z, y + z
//     - z, y + z, x + y + 2 * z
//   - Swap from left:
//     - x, y    , z
//     - y, x + y, z
//     - y, z    , x + y + z
//     - z, y + z, x + y + z
// - To minimize the largest element, we should swap from the left.

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();

        // Ignore first line
        io::stdin().read_line(&mut input).unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        let mut a: Vec<u64> = input
            .trim()
            .split(' ')
            .map(|ai| ai.parse().unwrap())
            .collect();

        for i in 0..(a.len() - 1) {
            if a[i] > a[i + 1] {
                (a[i], a[i + 1]) = (a[i + 1], a[i] + a[i + 1]);
            }
        }

        println!("{}", a[a.len() - 1]);
    }
}
