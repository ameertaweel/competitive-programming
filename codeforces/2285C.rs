const MOD: u64 = 1000000007;

fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let n: usize = next(&mut iter);

        let mut a: Vec<u64> = take(&mut iter, n).collect();

        a.sort_unstable();
        a.reverse();
        for i in 0..(a.len() - 1) {
            if a[i] == a[i + 1] {
                a.truncate(i);
                break;
            }
        }

        let a = a;

        let mut distinct = 1;
        for i in 0..a.len() {
            distinct += fast_modular_exponentiation(a[i] - 1, i as u64 + 1, MOD);
            distinct %= MOD;
        }

        println!("{}", distinct);
    }
}

fn fast_modular_exponentiation(base: u64, power: u64, modulus: u64) -> u64 {
    let mut powers = vec![base];
    let mut trgt_power = power;
    let mut last_power = 1;
    let mut last_index = 0;

    while last_power < trgt_power {
        let last_value = powers[last_index];

        powers.push((last_value * last_value) % modulus);
        last_power *= 2;
        last_index += 1;
    }

    let mut result = 1;

    while trgt_power > 0 {
        if last_power > trgt_power {
            last_power /= 2;
            last_index -= 1;
            continue;
        }

        result *= powers[last_index];
        result %= modulus;
        trgt_power -= last_power;
    }

    return result;
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
