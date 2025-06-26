use std::io::{self, BufRead};

fn main() {
    let n: usize = read_usize();
    let mut A: Vec<u64> = read_u64_vec(n);
    let key = partition(n, &mut A);
    for i in 0..n {
        if i==0 { print!("{}", A[i]); }
        else if i==key { print!(" [{}]", A[i]); }
        else if i==n-1 { println!(" {}", A[i]); }
        else { print!(" {}", A[i]); }
    }
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
        .map(|s| s.parse().unwrap() )
        .collect()
}

fn partition(n: usize, A: &mut Vec<u64>) -> usize{
    let x = A[n-1];
    let mut i: usize = 0;
    let mut tmp: u64 = 0;
    for j in 0..n-1 {
        if A[j]<=x {
            i += 1;
            tmp = A[i-1];
            A[i-1] = A[j];
            A[j] = tmp;
        }
    }
    tmp = A[i];
    A[i] = A[n-1];
    A[n-1] = tmp;
    i
}