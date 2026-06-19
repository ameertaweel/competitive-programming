use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();

        // Ignore first line
        io::stdin().read_line(&mut input).unwrap();
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        let a: Vec<u32> = input
            .trim()
            .split(' ')
            .map(|ai| ai.parse().unwrap())
            .collect();

        let mut min = a[0];
        let mut sum = 0;

        for ai in a.into_iter() {
            if ai < min {
                min = ai;
            }
            sum += min;
        }

        println!("{}", sum);
    }
}
