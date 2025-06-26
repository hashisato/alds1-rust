use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<u64> = input(num);
    for i in 0..num {
        print!("{}", data[i]);
        if i != num-1 {
            print!(" ");
        }
    }
    println!("");
    sort(num, data);
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

fn sort(num: usize, mut data: Vec<u64>) {
    let mut v: u64 = 0;
    let mut j: usize = 0;
    for i in 1..num {
        v = data[i];
        j = i;
        while j>0 && data[j-1]>v {
            data[j] = data[j-1];
            j = j-1; 
        }
        data[j] = v;
        for i in 0..num {
            print!("{}", data[i]);
            if i != num-1 {
                print!(" ");
            }
        }
        println!("");
    }
}