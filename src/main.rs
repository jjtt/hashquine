use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let count = 9;
    let func = lause;
    let combinations = (1..=count).map(|_| 0..16u8).multi_cartesian_product();
    combinations
        .into_par_iter()
        .map(|c| {
            let s = func(&c);
            (c, s)
        })
        .map(|(c, s)| (c, hash_half_bytes_as_vec(&s, count)))
        .filter(|(c, s)| c == s)
        .for_each(|(c, s)| println!("{}", func(&c)));
}

/// a const array of 16 strings
const ENGLISH: [&str; 16] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "a", "b", "c",
    "d", "e", "f",
];
/// a const array of 16 strings
const FINNISH: [&str; 16] = [
    "nolla",
    "yksi",
    "kaksi",
    "kolme",
    "neljä",
    "viisi",
    "kuusi",
    "seitsemän",
    "kahdeksan",
    "yhdeksän",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
];

fn sentence(input: &Vec<u8>) -> String {
    format!(
        "The SHA256 for this sentence begins with: {}, {}, {}, {}, {}, {} and {}.",
        ENGLISH[input[0] as usize],
        ENGLISH[input[1] as usize],
        ENGLISH[input[2] as usize],
        ENGLISH[input[3] as usize],
        ENGLISH[input[4] as usize],
        ENGLISH[input[5] as usize],
        ENGLISH[input[6] as usize],
    )
}
fn lause(input: &Vec<u8>) -> String {
    format!(
        "Tämän lauseen SHA256 alkaa: {}, {}, {}, {}, {}, {}, {}, {} ja {}.",
        FINNISH[input[0] as usize],
        FINNISH[input[1] as usize],
        FINNISH[input[2] as usize],
        FINNISH[input[3] as usize],
        FINNISH[input[4] as usize],
        FINNISH[input[5] as usize],
        FINNISH[input[6] as usize],
        FINNISH[input[7] as usize],
        FINNISH[input[8] as usize],
    )
}

/// return first 7 half bytes from sha256 hash of input
fn hash_half_bytes_as_vec(input: &str, i: usize) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let foo = result.as_slice();
    let mut out = Vec::with_capacity(i);
    for c in foo {
        out.push(((c & 0xf0) >> 4));
        if out.len() == i {
            break;
        }
        out.push((c & 0x0f));
        if out.len() == i {
            break;
        }
    }
    out
}
