use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let p: Vec<f64> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let q: Vec<f64> = lines.next().unwrap().unwrap()        .split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut e = vec![vec![0.0; n + 2]; n + 2];
    let mut w = vec![vec![0.0; n + 2]; n + 2];

    for i in 0..=n {
        e[i+1][i] = q[i];
        w[i+1][i] = q[i];
    }

    for l in 1..=n {
        for i in 1..=n - l + 1 {
            let j = i + l - 1;
            e[i][j] = std::f64::INFINITY;
            w[i][j] = w[i][j-1] + p[j-1] + q[j];
            for r in i..=j {
                let t = e[i][r-1] + e[r+1][j] + w[i][j];
                if t < e[i][j] {
                    e[i][j] = t;
                }
            }
        }
    }

    println!("{:.8}", e[1][n]);
}
