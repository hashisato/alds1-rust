use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (n, k, mut w) = input();
    println!("{}", allocation(n, k, &mut w));
}

fn input() -> (usize, usize, Vec<usize>) {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let line = lines.next().unwrap().unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();
    let n: usize = words[0].parse().expect(&format!("Failed to parse n from: '{}'", line));
    let k: usize = words[1].parse().expect(&format!("Failed to parse k from: '{}'", line));

    let mut w: Vec<usize> = Vec::new();
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let value: usize = line.trim().parse().expect(&format!("Failed to parse value {} from: '{}'", i, line.trim()));
        w.push(value);
    }
    (n, k, w)
}

fn allocation(n: usize, k: usize, w: &mut Vec<usize>) -> usize {
    let mut left = *w.iter().max().unwrap();
    let mut right = w.iter().sum::<usize>();
    let mut mid = 0;
    
    while left < right {
        mid = (left + right) / 2;
        if canLoad(n, k, w, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn canLoad(n: usize, k: usize, w: &Vec<usize>, capacity: usize) -> bool {
    let mut trucks_used = 1;
    let mut current_weight = 0;
    
    for i in 0..n {
        if current_weight + w[i] <= capacity {
            current_weight += w[i];
        } else {
            trucks_used += 1;
            current_weight = w[i];
            if trucks_used > k {
                return false;
            }
        }
    }
    true
}