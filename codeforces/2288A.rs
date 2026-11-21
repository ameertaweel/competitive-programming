fn main() {
    let input = read_all_input();

    let mut iter = input.split_whitespace();

    let t: usize = next(&mut iter);

    for _ in 0..t {
        let x: u64 = next(&mut iter);

        let min_stack = sum_n_gte_binary_search(x);
        let max_stack = x;

        let mut min_moves = moves_from_stack(x, min_stack);

        for stack in (min_stack + 1)..=max_stack {
            let moves = moves_from_stack(x, stack);
            if moves <= min_moves {
                min_moves = moves;
                continue;
            }
            break;
        }

        println!("{}", min_moves);
    }
}

fn moves_from_stack(x: u64, stack: u64) -> u64 {
    let charges = stack - 1;

    let damage_max = (stack * (stack + 1)) / 2;
    let damage_rem = damage_max - x;

    let attacks_max = stack;
    let attacks_rem = sum_n_lte_binary_search(damage_rem);

    let attacks = attacks_max - attacks_rem;

    return charges + attacks;
}

fn sum_n_gte_binary_search(target: u64) -> u64 {
    let mut a = 0;
    let mut b = target;

    while a < b {
        let mid = a + (b - a) / 2;

        let res = mid * (mid + 1) / 2;
        if res == target {
            return mid;
        }

        if res > target {
            b = mid;
        } else {
            a = mid + 1;
        }
    }

    return b;
}

fn sum_n_lte_binary_search(target: u64) -> u64 {
    let mut a = 0;
    let mut b = target;

    while a < b {
        let mid = a + (b - a + 1) / 2;

        let res = mid * (mid + 1) / 2;
        if res == target {
            return mid;
        }

        if res > target {
            b = mid - 1;
        } else {
            a = mid;
        }
    }

    return a;
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
