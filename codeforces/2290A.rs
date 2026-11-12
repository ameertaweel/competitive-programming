fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let mut piles: Vec<u32> = take(&mut iter, 3).collect();
        piles.sort_unstable();
        let piles = piles;

        if piles[2] == piles[1] + 1 && piles[1] == piles[0] + 1 {
            println!("YES");
        } else {
            println!("NO");
        }
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
