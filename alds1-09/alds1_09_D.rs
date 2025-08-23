use std::io::*;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut a: Vec<i32> = vec![];
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    for s in buf.trim().split_whitespace() {
        a.push(s.parse().unwrap());
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