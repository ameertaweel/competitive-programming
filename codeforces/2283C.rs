use std::collections::HashSet;
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
        let n: u32 = buf.trim().parse().unwrap();

        let mut max_power_of_two = 1;
        while 2 * max_power_of_two <= n {
            max_power_of_two *= 2;
        }

        let mut m = 0;
        for i in 1..=n {
            m ^= i;
        }

        let mut doubled = HashSet::new();

        while max_power_of_two > 0 {
            if m ^ max_power_of_two ^ (2 * max_power_of_two) > m {
                m = m ^ max_power_of_two ^ (2 * max_power_of_two);
                doubled.insert(max_power_of_two);
            }
            max_power_of_two /= 2;
        }

        for i in 1..n {
            if doubled.contains(&i) {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        if doubled.contains(&n) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
