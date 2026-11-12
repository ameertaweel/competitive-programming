use std::collections::HashMap;

fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let n: usize = next(&mut iter);

        let h: Vec<u32> = take(&mut iter, n).collect();

        let mut counts = HashMap::new();
        for &hi in &h {
            let count = counts.entry(hi).or_insert(0);
            *count += 1;
        }
        let counts = counts;

        let mut uniq: Vec<_> = counts.keys().map(|k| *k).collect();
        uniq.sort_unstable();
        let uniq = uniq;

        let mut order = vec![0; n];
        let mut a = 0;
        let mut b = n - 1;
        let mut visible = 0;

        for &value in &uniq {
            let mut count = *counts.get(&value).unwrap();

            visible += 1;

            if count > 1 {
                visible += 1;
                order[b] = value;
                b -= 1;
                count -= 1;
            }

            for _ in 0..count {
                order[a] = value;
                a += 1;
            }
        }

        println!("{}", visible);
        for i in 0..(n - 1) {
            print!("{} ", order[i]);
        }
        println!("{}", order[n - 1]);
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

fn take<'a, R, I, S>(iter: &'a mut I, n: usize) -> impl Iterator<Item = R> + 'a
where
    R: std::str::FromStr,
    <R as std::str::FromStr>::Err: std::fmt::Debug,
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    iter.take(n).map(|s| s.as_ref().parse().unwrap())
}
