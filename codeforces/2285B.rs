fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let (n, k): (usize, usize) = (next(&mut iter), next(&mut iter));

        let s: String = next(&mut iter);

        let ones: usize = s.chars().filter(|&c| c == '1').count();
        let zeroes = n - ones;

        let (min, max) = if ones < zeroes {
            (ones, zeroes)
        } else {
            (zeroes, ones)
        };

        let k = std::cmp::min(k, min);

        let (min, max) = (min - k, max + k);

        let diversity_min = if min <= 1 { min } else { min * (min + 1) / 2 };
        let diversity_max = if max <= 1 { max } else { max * (max + 1) / 2 };

        let diversity = diversity_min + diversity_max + 1;

        println!("{}", diversity);
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
