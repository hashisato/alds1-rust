use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<(char, u64)> = input(num);
    print(data);
    bubbleSort(num, data);
    //selectionSort(num, &mut data);
}

fn input(num: usize) -> Vec<(char, u64)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let mut data: Vec<(char, u64)> = Vec::new();
    for i in 0..num {
        let mut element: (char, u64) = Default::default();
        let s = words[i];
        if s.len() == 2 {
            let mark = s.chars().next().unwrap();
            let number_str = &s[1..];
            let number = u64::from_str(number_str).unwrap();
            element = (mark, number);
        }
        else {
            panic!("Invalid input format: {}", s);
        }
        data.push(element);
    }
    data
}

fn bubbleSort(num: usize, data: Vec<(char, u64)>) {
    let mut flag: bool = true;
    let mut count: usize = 0;
    while flag {
        flag = false;
        for i in (1..num).rev() {
            if data[i].1 < data[i-1].1 {
                let tmp1: (char, u64) = data[i].clone();
                let tmp2: (char, u64) = data[i-1].clone();
                data[i] = tmp2;
                data[i-1] = tmp1;
                flag = true;
                count += 1;
            }
        }
    }
    println!("\n{}", count);
}

/*
fn selectionSort(num: usize, data: &mut Vec<Trump>) {
    let mut count: usize = 0;
    for i in 0..num {
        let mut minj: usize = i;
        for j in i..num {
            if data[j].number<data[minj].number {
                minj = j;
            }
        }
        if i!=minj {
            let tmp1: Trump = data[i].clone();
            let tmp2: Trump = data[minj].clone();
            data[i] = tmp2;
            data[minj] = tmp1;
            count += 1;
        }
    }
    println!("\n{}",count);
}
*/

fn print(data: Vec<(char, u64)>) {
    for i in 0..data.len() {
        print!("{}{}", data[i].0,data[i].1);
        if i != data.len()-1 {
            print!(" ");
        }
    }
}