use std::io::{self, BufRead};

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<u64> = input(num);
    let ans: i64 = max_prof(data, num);
    println!("{}",ans);
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

fn max_prof(data: Vec<u64>, num: usize) -> i64 {
    let mut minv: i64 = data[0] as i64;
    let mut maxv: i64 = (data[1]-data[0]) as i64;
    for i in 1..num {
        let j: i64 = data[i] as i64; 
        if maxv<j-minv {
            maxv = j-minv;
        }
        if j<minv {
            minv = j;
        }
    }
    maxv
}