use std::vec::Vec;

fn reverse_section(list: &mut [u8], start: usize, len: usize) {
    if len <= 1 { return; }
    for n in 0..len/2 {
        let i = (start + n) % list.len();
        let j = (start + len - n - 1) % list.len();
        list.swap(i, j);
    }
}

pub fn knot_hash_raw(list: &mut [u8], lengths: &[usize], rounds: usize) {
    let mut current_pos = 0usize;
    let mut skip_size = 0usize;
    //println!("{list:?}");
    for _ in 0..rounds {
        for l in lengths {
            reverse_section(list, current_pos, *l);
            //println!("{list:?}");
            current_pos += l + skip_size;
            current_pos %= list.len();
            skip_size += 1;
        }
    }
}

pub fn knot_hash(input: &str) -> Vec<u8> {
    let mut lengths: Vec<usize> = input.chars().map(|c| c as usize).collect();
    lengths.extend_from_slice(&[17,31,73,47,23]);
    let mut list = Vec::from_iter(0u8..=255);
    knot_hash_raw(&mut list, &lengths, 64);
    list.chunks(16)
        .map(|slice| slice.iter().copied().reduce(|acc, i| acc ^ i).unwrap())
        .collect()
}
