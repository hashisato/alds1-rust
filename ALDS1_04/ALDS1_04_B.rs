use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (n, mut S) = input();
    let (q, mut T) = input();
    let mut count = 0;
    for i in T {
        count += binarysearch(n, &mut S, i);
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

fn binarysearch(num: usize, vec: &mut Vec<u64>, key: u64) -> u64 {
    let mut i: usize = 0;
    let mut right: usize = num;
    let mut left: usize = 0;
    while left<right {
        let mid: usize = (right+left)/2;
        if vec[mid]==key {
            return 1;
        }
        else if key<vec[mid] {
            right = mid;
        }
        else {
            left = mid+1;
        }
    }
    0
}