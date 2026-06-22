// The greedy algorithm used to select the matching bj for each ai is:
// - For each ai, find the smalles bj >= ai.
// - Perform stage one (making ai equal to bj).
// - Remove bj from the set of possible candidates (no bi can be used twice).
//
// This works because let's say I assign bj to ak (not ai),
// And then I assign bl to ai (l > j).
// And that this assignment is optimal.
// Since b is sorted, this means bl > bj. And I can assign bl to ak instead.
// But why is this new assignment optimal?
// I save (l - j) swaps by assigning bj to ai instead of bl;
// The amount of swaps I incur for moving ak to bl depends on ak's location
// relative to bl, but it's never more than (l - j).

use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    'outer: for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut a: Vec<u64> = input
            .trim()
            .split(' ')
            .map(|ai| ai.parse().unwrap())
            .collect();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut b: HashSet<u64> = input
            .trim()
            .split(' ')
            .map(|bi| bi.parse().unwrap())
            .collect();

        for i in 0..n {
            let mut min_fitting_bi = u64::MAX;

            for &bi in &b {
                if bi < a[i] {
                    // bi doesn't fit ai
                    continue;
                }

                if bi < min_fitting_bi {
                    min_fitting_bi = bi;
                }
            }

            if min_fitting_bi == u64::MAX {
                println!("{}", -1);
                continue 'outer;
            }

            a[i] = min_fitting_bi;

            b.remove(&min_fitting_bi);
        }

        let mut swaps = 0;
        loop {
            let mut did_swap = false;
            for i in 0..(n - 1) {
                if a[i] > a[i + 1] {
                    (a[i + 1], a[i]) = (a[i], a[i + 1]);
                    did_swap = true;
                    swaps += 1;
                }
            }
            if !did_swap {
                break;
            }
        }

        println!("{}", swaps);
    }
}
