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
        let s: Vec<char> = buf.trim().chars().collect();

        if n < 2 * k {
            println!("-1");
            continue;
        }

        let mut prefix_flips = 0;
        for i in 0..k {
            if s[i] == 'L' {
                prefix_flips += 1;
            }
        }

        let mut left_so_far = 0;
        let mut suffix_flips = 0;
        for i in (k..n).rev() {
            if s[i] == 'R' {
                suffix_flips += 1;
            }
            left_so_far += 1;

            if left_so_far >= k {
                break;
            }
        }

        println!("{}", prefix_flips + suffix_flips);
    }
}
