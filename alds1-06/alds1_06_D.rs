use std::io::{self, BufRead};
use std::io::prelude::*;
use std::collections::HashMap;

const MAX_INDEX: usize = 1e4 as usize + 1;

fn main() {
    let n: usize = read_usize();
    let A: Vec<u64> = read_u64_vec(n);
    let weight = weigh(&A, n);
    println!("{}", weight);
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

fn print(vec: &Vec<u64>) {
    for i in vec {
        println!("{} ", i);
    }
}

fn weigh(W: &Vec<u64>, n: usize) -> u64 {
    let mut indexes: HashMap<u64, isize> = HashMap::new();
    let mut A = W.clone();
    let mut weight = 0;

    A.sort();
    let lowestwv = A[0];
    
    for (i, &w) in W.iter().enumerate() {
        indexes.insert(w, i as isize);
    }

    for i in 0..n {
        if *indexes.get(&W[i]).unwrap() < 0 {
            continue;
        }

        let fwv = W[i];
        let mut av = A[i];

        if fwv == av {
            continue;
        }

        let mut minwv = fwv;
        let mut next_idx = i;
        let mut sum1 = 0;
        let mut count = 1;

        while {
            sum1 += W[next_idx];

            if minwv > W[next_idx] {
                minwv = W[next_idx];
            }

            if fwv == av {
                false
            } else {
                count += 1;
                next_idx = *indexes.get(&av).unwrap() as usize;
                indexes.insert(av, -1);
                av = A[next_idx];
                true
            }
        } {}

        let sum2 = sum1 + minwv + (count + 1) * lowestwv;

        sum1 += (count - 2) * minwv;
        weight += sum1.min(sum2);
    }
    weight
}