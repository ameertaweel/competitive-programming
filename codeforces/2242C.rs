use std::io;

fn read_line(buf: &mut String) {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
}

fn main() {
    let mut buf = String::new();

    read_line(&mut buf);
    let t: usize = buf.trim().parse().unwrap();

    for _ in 0..t {
        read_line(&mut buf);
        let nk: Vec<usize> = buf.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        let (n, k) = (nk[0], nk[1]);

        read_line(&mut buf);
        let a: Vec<u64> = buf
            .trim()
            .split(' ')
            .map(|ai| ai.parse().unwrap())
            .collect();

        // Get repetition count of each unique number.
        // Sorted in non-decreasing order.
        let mut counts = vec![1];
        for i in 1..n {
            if a[i] != a[i - 1] {
                counts.push(0);
            }
            let last = counts.len() - 1;
            counts[last] += 1;
        }
        counts.sort_unstable();

        let mut arrays_of_size_k = 0;
        let mut count_last = 0;
        let mut dropped = 0;

        for i in 0..counts.len() {
            let count_curr = counts[i];
            if count_curr == count_last {
                continue;
            }

            // Remaining unique numbers
            let rem = counts.len() - i;

            dropped += rem * (count_curr - count_last);

            count_last = count_curr;

            if k < (n + rem - dropped) {
                continue;
            }
            if (k - (n + rem - dropped)) % rem != 0 {
                continue;
            }
            arrays_of_size_k += 1;
        }

        println!("{}", arrays_of_size_k);
    }
}
