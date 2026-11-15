fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let n: usize = next(&mut iter);

        if n < 4 {
            println!("-1");
            continue;
        }

        print!("4 3 2 1 ");
        for i in 4..n {
            print!("{} ", i + 1);
        }
        println!("");
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
