use std::io::{self, BufRead};

fn main() {
    let n = read_usize();
    let A = read_u64_vec(n);
    let q = read_usize();
    let m = read_u64_vec(q);
    makeCombination(A, m, q);
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

fn makeCombination(A: Vec<u64>, m: Vec<u64>, q: usize) {
    for i in m {
        let mut flag = false;
        for j in 0..q {
            flag = rec(&A, i, j);
            if flag { 
                println!("yes");
                break;
            }
        }
        if !flag { println!("no"); }
    }
}

fn rec(A: &Vec<u64>, rest: u64, start: usize) -> bool {
    if rest==0 { return true; }
    else if start>=A.len() { return false; }
    let result1 = rec(&A, rest, start+1);
    let result2 = if rest>=A[start]{
        rec(&A, rest-A[start], start+1)
    } else { false };
    result1||result2
}