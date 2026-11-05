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
        let n: usize = buf.trim().parse().unwrap();

        read_line(&mut buf);
        let mut a: Vec<u32> = buf.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        a.sort_unstable();
        let a = a;

        let mut max_multiples = 1;
        for i in 0..n {
            let ai = a[i];
            let mut multiples = 1;

            for j in i + 1..n {
                if a[j] % ai == 0 {
                    multiples += 1;
                }
            }

            if multiples > max_multiples {
                max_multiples = multiples;
            }
        }

        println!("{}", n - max_multiples);
    }
}
