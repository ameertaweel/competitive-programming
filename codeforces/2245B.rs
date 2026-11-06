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

fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let (n, c): (usize, i64) = (next(&mut iter), next(&mut iter));

        let mut a: Vec<i64> = take(&mut iter, n).map(|ai: i64| ai - c).collect();
        a.sort_unstable();
        a.reverse();
        let a = a;

        let negative_count = a.iter().filter(|&&ai| ai < 0).count();
        let non_negative_count = n - negative_count;
        let unmatched_negative_count = if negative_count <= non_negative_count {
            0
        } else {
            negative_count - non_negative_count
        };

        let take_largest_count = non_negative_count + unmatched_negative_count.div_ceil(2);

        let ans: i64 = a.iter().take(take_largest_count).sum();
        println!("{}", ans);
    }
}
