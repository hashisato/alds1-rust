use std::io::*;
use std::str::FromStr;

fn read_val<T: FromStr>() -> T {
    use std::io::{self, Read};
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().expect("parse error")
}

fn main() {
    let n: usize = read_val();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(read_val());
    }

    a.sort();

    for i in 0..n - 1 {
        let mut j = i;
        while j > 0 {
            let k = (j - 1) / 2;
            a.swap(j, k);
            j = k;
        }
        a.swap(0, i + 1);
    }

    let result: Vec<String> = a.iter().map(|v| v.to_string()).collect();
    println!("{}", result.join(" "));
}