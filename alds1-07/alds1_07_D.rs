use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let n: usize = read_usize();
    let preorder: Vec<usize> = read_vec(n);
    let inorder: Vec<usize> = read_vec(n);
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_vec(n: usize) -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim()
         .split_whitespace()
         .map(|s| usize::from_str(s).unwrap())
         .collect::<Vec<usize>>()
}