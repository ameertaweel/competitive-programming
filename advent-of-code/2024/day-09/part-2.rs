use std::io;
use std::collections::VecDeque;

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

    // Index, Size, Name
    let mut files: Vec<(u64, u64, u64)> = Vec::new();
    // Index, Size
    let mut gaps: VecDeque<(u64, u64)> = VecDeque::new();

    let mut idx = 0;
    for (i, size) in compact_disk_map.iter().enumerate() {
        if i % 2 == 0 {
            files.push((idx, *size, (i as u64) / 2));
        } else {
            gaps.push_back((idx, *size));
        }
        idx += size;
    }

    for file in files.iter_mut().rev() {
        let (ref mut file_idx, ref file_size, _) = file;
        let mut done = false;
        for _ in 0..gaps.len() {
            let (gap_idx, gap_size) = gaps.pop_front().unwrap();
            if gap_idx >= *file_idx { done = true; }
            if !done && gap_size >= *file_size {
                done = true;
                *file_idx = gap_idx;
                if gap_size > *file_size {
                    gaps.push_back((gap_idx + *file_size, gap_size - *file_size));
                }
            } else {
                gaps.push_back((gap_idx, gap_size));
            }
        }
    }

    let mut checksum = 0;
    for file in files.iter() {
        let (ref idx, ref size, ref name) = file;
        checksum += *name * (*idx..(*idx + *size)).sum::<u64>();
    }

    println!("{}", checksum);

    
//    let mut expanded_disk_map: Vec<i64> = compact_disk_map
//        .iter()
//        .enumerate()
//        .map(|(idx, cnt)| -> Vec<i64> {
//            let el: i64 = if idx % 2 == 0 { (idx / 2) as i64 } else { -1 };
//            vec![el; *cnt as usize]
//        })
//        .flatten()
//        .collect();
//
//    // Two-Pointer Method
//    let mut i = 0;
//    let mut j = expanded_disk_map.len() - 1;
//    while i < j {
//        if expanded_disk_map[i] != -1 {
//            i += 1;
//            continue;
//        }
//        if expanded_disk_map[j] == -1 {
//            j -= 1;
//            continue;
//        }
//        expanded_disk_map.swap(i, j);
//        i += 1;
//        j -= 1;
//    }
//
//    let mut checksum = 0;
//    for (idx, val) in expanded_disk_map.iter().enumerate() {
//        if *val == -1 { break; }
//        checksum += (idx as u64) * (*val as u64);
//    }
//
//    println!("{}", checksum);
}
