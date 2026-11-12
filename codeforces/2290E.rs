use std::collections::VecDeque;

fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let (n, k): (usize, usize) = (next(&mut iter), next(&mut iter));

        let piles: Vec<u32> = take(&mut iter, n).collect();
        let mut drop = VecDeque::new();

        let mut total = 0;
        let mut last_k = 0;
        for i in (0..n).rev() {
            let pile = piles[i] + last_k;
            total += pile;
            last_k += pile;
            drop.push_front(pile);
            if drop.len() > k {
                let dropped = drop.pop_back().unwrap();
                last_k -= dropped;
            }
        }

        if total % 2 == 0 {
            println!("Bob");
        } else {
            println!("Alice");
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
