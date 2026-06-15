use std::io;

fn main() {
    let mut input = String::new();

    // Ignore first line
    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let is_hard = input.contains('1');

    if is_hard {
        println!("HARD");
    } else {
        println!("EASY");
    }
}
