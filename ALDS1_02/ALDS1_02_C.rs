use std::io::{self, BufRead};
use std::str::FromStr;

enum Trump {
    Mark(char, u64),
    Number(u64),
}

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<Trump> = input(num);
    print(data);
    //bubbleSort(num, data);
    //selectionSort(num, data);
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
                data.push(Trump::Mark(mark, number));
            } else {
                panic!("Invalid number format: {}", number_str);
            }
        } else if let Ok(number) = u64::from_str(s) {
            data.push(Trump::Number(number));
        } else {
            panic!("Invalid input format: {}", s);
        }
    }
    data

    /*
    for i in 0..num {
        let x: u64 = u64::from_str(words[i]).unwrap();
        data.push(x);
    }
    data
    */
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
    //print(data);
    println!("\n{}",count);
}

fn selectionSort(num: usize, mut data: Vec<u64>) {
    let mut count: usize = 0;
    for i in 0..num {
        let mut minj: usize = i;
        for j in i..num {
            if data[j]<data[minj] {
                minj = j;
            }
        }
        if i!=minj {
            let tmp: u64 = data[i];
            data[i] = data[minj];
            data[minj] = tmp;
            count += 1;
        }
    }
    //print(data);
    println!("\n{}",count);
}

fn print(data: Vec<Trump>) {
    for i in 0..data.len() {
        print!("{:?}", data[i]:Number);
        if i != data.len()-1 {
            print!(" ");
        }
    }
}