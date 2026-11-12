use std::cmp::{max, min};
use std::collections::HashSet;

fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let n: usize = next(&mut iter);

        let s: String = next(&mut iter);
        let s: Vec<_> = s.chars().collect();

        let mut u = HashSet::new();

        for c in ('a' as u8)..=('z' as u8) {
            u.insert(c as u8);
        }

        let original = u.len();

        let mut a = 0;
        let mut b = n - 1;

        while a < b {
            let ca = s[a] as u8;
            let cb = s[b] as u8;

            let (cmin, cmax) = (min(ca, cb), max(ca, cb));

            for i in (cmin + 1)..=cmax {
                u.remove(&i);
            }

            a += 1;
            b -= 1;
        }

        println!("{}", original - u.len());
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
