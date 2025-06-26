use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut input = lines.next().unwrap().unwrap();
    let mut stack: Vec<i64> = Vec::new();
    for symbol in input.split_whitespace() {
        match symbol {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a+b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a-b);    
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a*b);       
            }
            num => {
                let n = num.parse::<i64>().unwrap();
                stack.push(n);
            }
        }
    }
    let result = stack.pop().unwrap();
    println!("{}", result);
}