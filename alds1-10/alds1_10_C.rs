use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let num: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..num {
        let x = lines.next().unwrap().unwrap();
        let y = lines.next().unwrap().unwrap();
        let res = lcs_length(&x, &y);
        println!("{}", res);
    }
}

fn lcs_length(x: &str, y: &str) -> usize {
    let m = x.len();
    let n = y.len();
    let x_bytes = x.as_bytes();
    let y_bytes = y.as_bytes();
    let mut c = vec![vec![0; n+1]; m+1];

    for i in 1..=m {
        for j in 1..=n {
            if x_bytes[i-1] == y_bytes[j-1] {
                c[i][j] = c[i-1][j-1] + 1;
            } else {
                c[i][j] = c[i-1][j].max(c[i][j-1]);
            }
        }
    }
    c[m][n]
}
