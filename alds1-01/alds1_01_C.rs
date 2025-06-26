use std::io::{self, BufRead};

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<u64> = input(num);
    let mut count = 0;
    for &i in &data {
        if isPrime(i) {
            count += 1;
        }
    }
    println!("{}",count);
}

fn input(num: usize) -> Vec<u64> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut data: Vec<u64> = Vec::new();
    for i in 0..num {
        let line = lines.next().unwrap().unwrap();
        let x: u64 = line.trim().parse().unwrap();
        data.push(x);
    }
    data
}

fn isPrime(x: u64) -> bool {
    if x==2 {
        return true;
    }
    else if x<2 || x%2==0 {
        return false;
    }
    let limit: u64 = (x as f64).sqrt() as u64 + 1;
    let mut i = 3;
    while i<=limit {
        if x%i==0 {
            return false;
        }
        i += 2;
    }
    true
}