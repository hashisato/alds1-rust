use std::io::{self, BufRead};

fn main() {
    let n: usize = read_usize();
    let mut A: Vec<u64> = read_u64_vec(n);
    let mut B: Vec<u64> = vec![0; n];
    countingSort(n, &mut A, &mut B);
    for i in 0..B.len() {
        if i!=B.len()-1 {
            print!("{} ",B[i]);
        }else { println!("{}",B[i]); }
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

fn countingSort(n: usize, A: &mut Vec<u64>, B: &mut Vec<u64>) {
    let k = 10000;
    let mut C: Vec<u64> = vec![0; k+1];
    for &i in A.iter() { C[i as usize] += 1; }
    for i in 1..k { C[i] += C[i-1]; }
    for &i in A.iter().rev() {
        let a: usize = i as usize;
        C[a] -= 1;
        B[C[a] as usize] = i;
    }
}