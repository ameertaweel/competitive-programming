use std::io;

fn main() {
    let stdin = io::stdin();

    // Read Input
    let mut compact_disk_map = String::new();
    stdin.read_line(&mut compact_disk_map).unwrap();
    let compact_disk_map: Vec<u64> = compact_disk_map
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut expanded_disk_map: Vec<i64> = compact_disk_map
        .iter()
        .enumerate()
        .map(|(idx, cnt)| -> Vec<i64> {
            let el: i64 = if idx % 2 == 0 { (idx / 2) as i64 } else { -1 };
            vec![el; *cnt as usize]
        })
        .flatten()
        .collect();

    // Two-Pointer Method
    let mut i = 0;
    let mut j = expanded_disk_map.len() - 1;
    while i < j {
        if expanded_disk_map[i] != -1 {
            i += 1;
            continue;
        }
        if expanded_disk_map[j] == -1 {
            j -= 1;
            continue;
        }
        expanded_disk_map.swap(i, j);
        i += 1;
        j -= 1;
    }

    let mut checksum = 0;
    for (idx, val) in expanded_disk_map.iter().enumerate() {
        if *val == -1 { break; }
        checksum += (idx as u64) * (*val as u64);
    }

    println!("{}", checksum);
}
