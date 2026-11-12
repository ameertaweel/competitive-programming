fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let _: usize = next(&mut iter);

        let chars: String = next(&mut iter);
        let chars: Vec<_> = chars.chars().map(|c| c == 'R').collect();

        let mut current = true;
        let mut switches = 0;

        for &c in &chars {
            if c != current {
                current = c;
                switches += 1;
            }
            current = !current;
        }

        println!("{}", switches);
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
