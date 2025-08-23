use std::io::*;
use std::str::FromStr;

fn rin<T: FromStr>() -> T {
    let s = stdin();
    let s = s.lock();
    let s: String = s.bytes()
        .map(|c| c.expect("failed reading char.") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    s.parse().ok().expect("failed parsing.")
}

fn main() {
    let n: usize = rin();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(rin());
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