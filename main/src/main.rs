use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Clone)]
struct Trump {
    mark: char,
    number: u64,
}

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<Trump> = input(num);
    // print(data);
    bubbleSort(num, &mut data);
    // selectionSort(num, &mut data);
}

fn input(num: usize) -> Vec<Trump> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<Trump> = Vec::new();

    for i in 0..num {
        let s = words[i];
        if s.len() == 2 {
            let mark = s.chars().next().unwrap();
            let number_str = &s[1..];
            if let Ok(number) = u64::from_str(number_str) {
                data.push(Trump { mark, number });
            } else {
                panic!("Invalid number format: {}", number_str);
            }
        } else if let Ok(number) = u64::from_str(s) {
            data.push(Trump { mark: ' ', number });
        } else {
            panic!("Invalid input format: {}", s);
        }
    }
    data
}

fn bubbleSort(num: usize, data: &mut Vec<Trump>) {
    let mut flag: bool = true;
    let mut count: usize = 0;
    while flag {
        flag = false;
        for i in (1..num).rev() {
            if data[i].number < data[i - 1].number {
                let tmp1: Trump = data[i].clone();
                let tmp2: Trump = data[i - 1].clone();
                data[i] = tmp2;
                data[i - 1] = tmp1;
                flag = true;
                count += 1;
            }
        }
    }
    println!("\n{}", count);
}

fn selectionSort(num: usize, data: &mut Vec<u64>) {
    let mut count: usize = 0;
    for i in 0..num {
        let mut minj: usize = i;
        for j in i..num {
            if data[j] < data[minj] {
                minj = j;
            }
        }
        if i != minj {
            let tmp: u64 = data[i];
            data[i] = data[minj];
            data[minj] = tmp;
            count += 1;
        }
    }
    println!("\n{}", count);
}

fn print(data: &Vec<Trump>) {
    for i in 0..data.len() {
        print!("{}{}", data[i].mark, data[i].number);
        if i != data.len() - 1 {
            print!(" ");
        }
    }
}