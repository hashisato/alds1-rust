use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    dictionary();
}

fn dictionary() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let n: usize = word.trim().parse().unwrap();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut set: HashSet<String> = HashSet::new();
    for i in 0..n {
        let mut line = lines.next().unwrap().unwrap().trim().to_string();
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "insert" => {
                set.insert(words[1].to_string());
            },
            "find" => {
                if set.contains(&words[1].to_string()) {
                    println!("yes");
                } else {
                    println!("no");
                }
            },
            _ => {}
        }
    }
}