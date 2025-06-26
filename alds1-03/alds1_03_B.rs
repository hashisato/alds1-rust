use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (q, mut queue) = input();
    let mut total: i64 = 0;
    while let Some(mut process)=queue.pop_front() {
        if process.1>q {
            process.1 -= q;
            total += q;
            queue.push_back(process);
        } else {
            total += process.1;
            println!("{} {}", process.0, total);
        }
    }
}

fn input() -> (i64, VecDeque<(String, i64)>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap().trim().to_string();
    let words = first_line.split_whitespace().collect::<Vec<&str>>();
    let n: usize = usize::from_str(words[0]).unwrap();
    let q: i64 = i64::from_str(words[1]).unwrap();
    let mut data: VecDeque<(String, i64)> = VecDeque::new();
    for i in 0..n {
        let line = lines.next().unwrap().unwrap().trim().to_string();
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let name: String = String::from_str(words[0]).unwrap();
        let time: i64 = i64::from_str(words[1]).unwrap();
        data.push_back((name, time));
    }
    (q, data)
}