fn main() {
    let mut buf = String::new();

    read_line(&mut buf);
    let t: usize = buf.trim().parse().unwrap();

    for _ in 0..t {
        read_line(&mut buf);
        let n: u64 = buf.trim().parse().unwrap();

        let mut s = String::new();
        s.push_str("+");

        for _ in 1..n {
            s.push_str("--+");
        }

        println!("{}", s);

        read_line(&mut buf);
        let k: u64 = buf.trim().parse().unwrap();

        let y = if k == 0 {
            0
        } else if k == 1 {
            1
        } else {
            1 + (k - 1) / 3
        };

        let x = n - y;
        println!("{}", x + 2 * y);
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
