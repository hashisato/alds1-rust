use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<u64> = input(num);
    bubbleSort(num, data);
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

fn bubbleSort(num: usize, mut data: Vec<u64>) {
    let mut flag: bool = true;
    let mut count: usize = 0;
    while flag {
        flag = false;
        for i in (1..num).rev() {
            if data[i]<data[i-1] {
                let tmp: u64 = data[i];
                data[i] = data[i-1];
                data[i-1] = tmp;
                flag = true;
                count += 1;
            }
        }
    }
    print(data);
    println!("\n{}",count);
}

fn print(data: Vec<u64>) {
    for i in 0..data.len() {
        print!("{}", data[i]);
        if i != data.len()-1 {
            print!(" ");
        }
    }
}