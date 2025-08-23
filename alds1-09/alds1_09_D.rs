use std::io::*;
use std::str::FromStr;

fn main() {
    let mut stdin = stdin();
    let n: usize = stdin.read_line().unwrap().trim().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(stdin.read_line().unwrap().trim().parse().unwrap());
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