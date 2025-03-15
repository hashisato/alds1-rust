use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let n = read_usize();
    let s = read_u64_vec(n);
    let q = read_usize();
    let t = read_u64_vec(q);

    let s_set: HashSet<u64> = s.into_iter().collect();
    let count = t.iter().filter(|&x| s_set.contains(x)).count();

    println!("{}", count);
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_u64_vec(n: usize) -> Vec<u64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse().unwrap())
        .collect()
}