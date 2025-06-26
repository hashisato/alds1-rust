use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let n: usize = read_usize();
    let mut A: Vec<(char, u64, u64)> = input(n);
    quickSort(0,n-1, &mut A);
    is_stable(&A);
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn input(n: usize) -> Vec<(char, u64, u64)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut vec: Vec<(char, u64, u64)> = Vec::new();
    for i in 0..n {
        if let Some(Ok(line))=lines.next() {
            let mut parts = line.split_whitespace();
            let mark: char = parts.next().unwrap().chars().next().unwrap();
            let number = parts.next().unwrap().parse().unwrap();
            vec.push((mark, number, i as u64));
        }
    }
    vec
}

fn quickSort(p: usize, r: usize, A: &mut Vec<(char, u64, u64)>) {
    if p<r {
        let q: usize = partition(p, r, A);
        quickSort(p, q-1, A);
        quickSort(q+1, r, A);
    }
}

fn partition(p: usize, r:usize, A: &mut Vec<(char, u64, u64)>) -> usize{
    let x = A[r].1;
    let mut i: usize = p-1;
    for j in p..r {
        if A[j].1<=x {
            i += 1;
            A.swap(i as usize, j as usize);
        }
    }
    A.swap((i+1) as usize, r as usize);
    i+1
}

fn is_stable(vec: &Vec<(char, u64, u64)>) {
    for i in 0..vec.len()-1 {
        if vec[i].1==vec[i+1].1 && vec[i].2>vec[i+1].2 {
            println!("Not stable");
            print(&vec);
            return;
        }   
    }
    println!("Stable");
    print(&vec);
}

fn print(vec: &Vec<(char, u64, u64)>) {
    for i in vec {
        println!("{} {}", i.0, i.1);
    }
}