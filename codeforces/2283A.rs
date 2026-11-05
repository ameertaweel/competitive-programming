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
        let lr: Vec<i32> = buf.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        let (l, r) = (lr[0], lr[1]);

        if l.abs() == r {
            println!("0");
            continue;
        }

        if l >= 0 || r <= 0 {
            if l == 0 || r == 0 {
                println!("{}", r - l - 1);
                continue;
            }

            println!("{}", r - l);
            continue;
        }

        println!("{}", (l + r).abs() - 1);
    }
}
