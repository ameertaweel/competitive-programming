fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let n: usize = next(&mut iter);

        let p: Vec<u32> = take(&mut iter, n).collect();

        let mut closest_l = vec![0; n];
        let mut stack = vec![];
        for i in (0..n).rev() {
            while stack.len() > 0 && p[i] < p[stack[stack.len() - 1]] {
                let j = stack.pop().unwrap();
                closest_l[j] = j - i;
            }
            closest_l[i] = i + 1;
            stack.push(i);
        }
        closest_l[0] = 1;

        let mut closest_r = vec![0; n];
        let mut stack = vec![];
        for i in 0..n {
            while stack.len() > 0 && p[i] < p[stack[stack.len() - 1]] {
                let j = stack.pop().unwrap();
                closest_r[j] = i - j;
            }
            closest_r[i] = n - i;
            stack.push(i);
        }
        closest_r[n - 1] = 1;

        let mut good_subarrays = 0;
        for i in 0..n {
            let is_pre_lower = if i == 0 { true } else { p[i - 1] < p[i] };
            let is_pst_lower = if i == n - 1 { true } else { p[i + 1] < p[i] };

            if is_pre_lower && is_pst_lower {
                good_subarrays += 1;
                continue;
            }

            if !is_pre_lower && !is_pst_lower {
                good_subarrays += 2;
                continue;
            }

            if is_pst_lower {
                good_subarrays += closest_l[i];
            }

            if is_pre_lower {
                good_subarrays += closest_r[i];
            }
        }

        println!("{}", good_subarrays);
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
