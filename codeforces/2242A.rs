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
        let c: Vec<u64> = buf
            .trim()
            .split(' ')
            .map(|ck| ck.parse().unwrap())
            .collect();

        let mut count_twos = 0;
        for &ck in &c {
            if ck >= 3 {
                // We have at least three of the same character A.
                // We can use this to create two equal bigrams.
                // AAA (String) -> AA (Bigram 1) + AA (Bigram 2)
                println!("YES");
                continue 'outer;
            }

            if ck >= 2 {
                count_twos += 1;
                if count_twos >= 2 {
                    // We have two of character A and two of character B.
                    // We can use this to create two equal bigrams.
                    // ABAB (String) -> AB (Bigram 1) + AB (Bigram 2)
                    println!("YES");
                    continue 'outer;
                }
            }
        }

        println!("NO");
    }
}
