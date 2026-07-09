use std::io;

fn read_line(buf: &mut String) {
    buf.clear();
    io::stdin().read_line(buf).unwrap();
}

fn main() {
    let mut buf = String::new();

    read_line(&mut buf);
    let t: usize = buf.trim().parse().unwrap();

    'outer: for _ in 0..t {
        // Ignore first line
        read_line(&mut buf);

        read_line(&mut buf);
        let a: Vec<u64> = buf
            .trim()
            .split(' ')
            .map(|ai| ai.parse().unwrap())
            .collect();

        let n = a.len();

        let mut prefix_1 = Vec::with_capacity(n);
        let mut prefix_2 = Vec::with_capacity(n);

        let mut acc_1 = 0;
        let mut acc_2 = 0;

        // x marks the end of part 1.
        // y marks the end of part 2.
        let mut valid_x_with_min_prefix_2 = i64::MAX;

        // At least one element should be reserved for part 3.
        // Hence `i in 0..(n - 1)` and not `i in 0..n`.
        for i in 0..(n - 1) {
            acc_1 += if a[i] <= 1 { 1 } else { -1 };
            acc_2 += if a[i] <= 2 { 1 } else { -1 };
            prefix_1.push(acc_1);
            prefix_2.push(acc_2);

            if prefix_2[i] >= valid_x_with_min_prefix_2 {
                println!("YES");
                continue 'outer;
            }

            if prefix_1[i] >= 0 && prefix_2[i] < valid_x_with_min_prefix_2 {
                valid_x_with_min_prefix_2 = prefix_2[i];
            }
        }

        println!("NO");
    }
}
