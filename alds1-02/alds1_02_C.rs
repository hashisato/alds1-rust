use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<(char, u64)> = input(num);
    let mut data_bubble = bubbleSort(num, &mut data.clone());
    let mut data_selection = selectionSort(num, &mut data.clone());
    data.sort_by(|a, b| a.1.cmp(&b.1));
    is_stable(&data, data_bubble);
    is_stable(&data, data_selection);
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

fn bubbleSort(num: usize, v: &mut Vec<(char, u64)>) -> Vec<(char, u64)> {
    let mut flag: bool = true;
    while flag {
        flag = false;
        for i in (1..num).rev() {
            if v[i].1 < v[i-1].1 {
                let tmp: (char, u64) = v[i];
                v[i] = v[i-1];
                v[i-1] = tmp;
                flag = true;
            }
        }
    }
    v.to_vec()
}

fn selectionSort(num: usize, v: &mut Vec<(char, u64)>) -> Vec<(char, u64)> {
    for i in 0..num {
        let mut minj: usize = i;
        for j in i..num {
            if v[j].1<v[minj].1 {
                minj = j;
            }
        }
        if i!=minj {
            let tmp: (char, u64) = v[i];
            v[i] = v[minj];
            v[minj] = tmp;
        }
    }
    v.to_vec()
}

fn print(data: &Vec<(char, u64)>) {
    for i in 0..data.len() {
        print!("{}{}", data[i].0,data[i].1);
        if i != data.len()-1 {
            print!(" ");
        }
    }
    println!("");
}

fn is_stable(correct: &Vec<(char, u64)>, vec: Vec<(char, u64)>) {
    print(&vec);
    for i in 0..correct.len() {
        if correct[i].0!=vec[i].0 || correct[i].1!=vec[i].1 {
            println!("Not stable");
            return;
        }
    }
    println!("Stable");
}