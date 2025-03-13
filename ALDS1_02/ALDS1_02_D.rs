use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    let num: usize = word.trim().parse().unwrap();
    let mut data: Vec<u64> = input(num);
    shellSort(num, &mut data);
    for &i in &data {
        println!("{}",i);
    }
}

fn input(num: usize) -> Vec<u64> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut data: Vec<u64> = Vec::new();
    for i in 0..num {
        let line = lines.next().unwrap().unwrap();
        let x: u64 = line.trim().parse().unwrap();
        data.push(x);
    }
    data
}

fn shellSort(num: usize, A: &mut Vec<u64>) {
    let mut count = 0;
    let G = generate_g(num);
    for &i in &G {
        count += insertionSort(num, A, i as usize);
    }
    println!("{}",G.len());
    print(&G);
    println!("{}",count);
}

fn generate_g(num: usize) -> Vec<u64> {
    let mut g: Vec<u64> = Vec::new();
    let mut element: u64 = 1;
    while element<=num as u64 {
        g.push(element);
        element = element*3+1;
    }
    g.reverse();
    g
}

fn insertionSort(num: usize, A: &mut Vec<u64>, g: usize) -> usize {
    let mut count = 0;
    for i in g..A.len() {
        let v = A[i];
        let mut j = i;
        while j>=g && A[j-g]>v {
            A[j]=A[j-g];
            j -= g;
            count += 1;
        }
        A[j] = v;
    }
    count
}

fn print(data: &Vec<u64>) {
    for i in 0..data.len() {
        print!("{}", data[i]);
        if i != data.len()-1 {
            print!(" ");
        }
    }
    println!("");
}