use std::io::{self, BufRead};

fn main() {
    let mut stack: Vec<(char, u64)> = input();
    calc(&mut stack);
    print(&stack)
}

fn input() -> Vec<(char, u64)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut line = lines.next().unwrap().unwrap().trim().to_string();
    let words = line.chars().collect::<Vec<(char)>>();
    let mut stack: Vec<(char, u64)> = Vec::new();
    for symbol in words {
        match symbol {
            '\\' => { stack.push((symbol, 0)); },
            '/' => { stack.push((symbol, 0)); },
            '_' => { stack.push((symbol, 0)); },
            _ => {}
        }
    }
    stack
}

fn calc(stack: &mut Vec<(char, u64)>) {
    let mut count = 0;
    let mut process = 0;
    let mut min = 0;
    for i in 0..stack.len() {
        count = 0;
        if stack[i].0=='/' {
            for j in (0..i).rev() {
                if stack[j].0=='\\' {
                    count += 1;
                    process = j;
                    stack[i].0 = '_';
                    stack[j].0 = '_';
                    stack[j].1 = count;
                    if process<min {
                        for i in process+1..=min {
                            stack[process].1 += stack[i].1;
                            stack[i].1 = 0;
                        }
                    }
                    min = process;
                    break;
                }
                else if stack[j].0=='_' {
                    count += 1;
                }
            }
        }        
    }
}

fn print(vec: &Vec<(char, u64)>) {
    let mut total = 0;
    let mut area: Vec<u64> = Vec::new();
    for i in vec {
        if i.1!=0 {
            total += i.1;
            area.push(i.1);
        }
    }
    print!("{}\n{}",total,area.len());
    for i in 0..area.len() {
        print!(" {}", area[i]);
    }
    println!("");
}