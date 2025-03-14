use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.chars().collect::<Vec<char>>();
    let mut stack: Vec<i64> = Vec::new();
    for word in words {
        let symbol = word;
        match symbol {
            '\\' => {
                println!("OK");
            }
            _ => {}
        }
    }
}