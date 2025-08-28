use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let list: VecDeque<i64> = dll();
    print(&list);
}

fn dll() -> VecDeque<i64> {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let n: usize = word.trim().parse().unwrap();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut list: VecDeque<i64> = VecDeque::new();
    for i in 0..n {
        let line = lines.next().unwrap().unwrap().trim().to_string();
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "deleteFirst" => {
                list.pop_back();
            },
            "deleteLast" => {
                list.pop_front();
            },
            "insert" => {
                let cmd: i64 = i64::from_str(words[1]).unwrap();
                list.push_back(cmd);
            },
            "delete" => {
                let cmd: i64 = i64::from_str(words[1]).unwrap();
                if let Some(remove_index) = list.iter().rposition(|x| *x == cmd) {
                    list.remove(remove_index);
                }
            },
            _ => {}
        }
    }
    list
}

fn print(data: &VecDeque<i64>) {
    for i in (0..data.len()).rev() {
        print!("{}", data[i]);
        if i != 0 {
            print!(" ");
        }
    }
    println!("");
}