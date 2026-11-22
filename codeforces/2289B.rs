fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let n: usize = next(&mut iter);
        let k: usize = next(&mut iter);

        let a: Vec<u32> = take(&mut iter, n).collect();

        let mut last_idx = 0;
        let mut last_val = a[0];

        let mut count = 0;

        for i in 0..n {
            if a[i] == last_val {
                continue;
            }

            let diff = i - last_idx;

            if diff > (n - k) {
                count += diff - (n - k);
            }

            last_idx = i;
            last_val = a[i];
        }

        let diff = n - last_idx;

        if diff > (n - k) {
            count += diff - (n - k);
        }

        println!("{}", count);
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
