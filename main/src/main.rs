use std::io::{self, BufRead};
use std::str::FromStr;
use std::usize;

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<u64> = input(num);
    
}

fn input(num: usize) -> Vec<u64> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<u64> = Vec::new();
    for i in 0..num {
        let x: u64 = u64::from_str(words[i]).unwrap();
        data.push(x);
    }
    data
}