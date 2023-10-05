use itertools::Itertools;

fn main() {
    let combinations = (0..16).permutations(7);
    combinations
        .map(|c| {
            let s = sentence(&c);
            (c, s)
        })
        .map(|(c, s)| (c, hash_half_bytes_as_vec(&s)))
        .filter(|(c, s)| c == s)
        .for_each(|(c, s)| println!("{}", sentence(&c)));
}

/// a const array of 16 strings
const FOO: [&str; 16] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "a", "b", "c",
    "d", "e", "f",
];

fn sentence(input: &Vec<usize>) -> String {
    format!(
        "The SHA256 for this sentence begins with: {}, {}, {}, {}, {}, {} and {}.",
        FOO[input[0]],
        FOO[input[1]],
        FOO[input[2]],
        FOO[input[3]],
        FOO[input[4]],
        FOO[input[5]],
        FOO[input[6]],
    )
}

/// return first 7 chars from sha256 hash of input
fn hash(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = format!("{:x}", result);
    result[..7].to_string()
}

/// return first 7 half bytes from sha256 hash of input
fn hash_half_bytes_as_vec(input: &str) -> Vec<usize> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let result = format!("{:x}", result);
    result
        .chars()
        .take(7)
        .map(|c| c.to_digit(16).unwrap() as usize)
        .collect()
}
