use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (x, y) = input();
    gcd(x, y);
}

fn input() -> (u64, u64) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<u64> = Vec::new();
    let x: u64 = u64::from_str(words[0]).unwrap();
    let y: u64 = u64::from_str(words[1]).unwrap();
    (x, y)
}

fn gcd(mut x: u64, mut y: u64) {
    if x<y {
        let buf = x;
        x = y;
        y = buf;
    }
    while y>0 {
        let r = x%y;
        x = y;
        y = r;
    }
    println!("{}",x);
}