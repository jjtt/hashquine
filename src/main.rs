use itertools::Itertools;
use std::iter;
use std::ops::Range;

fn main() {
    let count = 7;
    let func = sentence;
    let combinations = (1..=count).map(|_| 0..16).multi_cartesian_product();
    combinations
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

fn sentence(input: &Vec<usize>) -> String {
    format!(
        "The SHA256 for this sentence begins with: {}, {}, {}, {}, {}, {} and {}.",
        ENGLISH[input[0]],
        ENGLISH[input[1]],
        ENGLISH[input[2]],
        ENGLISH[input[3]],
        ENGLISH[input[4]],
        ENGLISH[input[5]],
        ENGLISH[input[6]],
    )
}
fn lause(input: &Vec<usize>) -> String {
    format!(
        "Tämän lauseen SHA256 alkaa: {}, {}, {}, {}, {}, {}, {}, {}.",
        FINNISH[input[0]],
        FINNISH[input[1]],
        FINNISH[input[2]],
        FINNISH[input[3]],
        FINNISH[input[4]],
        FINNISH[input[5]],
        FINNISH[input[6]],
        FINNISH[input[7]],
    )
}

/// return first 7 half bytes from sha256 hash of input
fn hash_half_bytes_as_vec(input: &str, i: usize) -> Vec<usize> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = format!("{:x}", result);
    result
        .chars()
        .take(i)
        .map(|c| c.to_digit(16).unwrap() as usize)
        .collect()
}
