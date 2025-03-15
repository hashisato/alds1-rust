use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (n, mut S) = input();
    let (q, mut T) = input();
    let mut count = 0;
    S.push(0);
    for i in T {
        count += search(n, &mut S, i);
    }
    println!("{}",count);
}

fn input() -> (usize, Vec<u64>) {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut vec: Vec<u64> = Vec::new();
    for i in 0..num {
        let x: u64 = u64::from_str(words[i]).unwrap();
        vec.push(x);
    }
    (num, vec)
}

fn search(num: usize, vec: &mut Vec<u64>, key: u64) -> u64 {
    let mut i: usize = 0;
    vec[num] = key;
    while vec[i]!=key {
        i += 1;
        if i==num {
            return 0;
        }
    }
    1
}