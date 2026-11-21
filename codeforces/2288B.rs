fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let x: u64 = next(&mut iter);
        let y: u64 = next(&mut iter);
        let _: u64 = next(&mut iter); // m

        let s: String = next(&mut iter);

        let mut x = x;
        let mut y = y;

        let mut max = x + y;

        let mut last_inc = -1;
        for (i, c) in s.chars().enumerate() {
            if c == '+' {
                last_inc = i as i64;
            }
        }

        for (i, c) in s.chars().enumerate() {
            if x + y == 0 {
                break;
            }

            if (i as i64) > last_inc {
                break;
            }

            if c == '+' {
                // Increase
                if y > 0 {
                    y += 1;
                } else {
                    // x > 0 for sure
                    x -= 1;
                    y += 1;
                }
            } else {
                // Decrease
                if y > 1 || (y > 0 && x == 0) {
                    y -= 1;
                    x += 1;
                } else {
                    // x > 0 for sure
                    x -= 1;
                }
            }
            let sum = x + y;
            if sum > max {
                max = sum;
            }
        }

        println!("{}", max);
    }
}

fn read_all_input() -> String {
    use std::io::Read;

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

fn next<R, I, S>(iter: &mut I) -> R
where
    R: std::str::FromStr,
    <R as std::str::FromStr>::Err: std::fmt::Debug,
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    iter.next().unwrap().as_ref().parse().unwrap()
}
