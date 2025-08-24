use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut p = vec![0; num+1];

    for k in 1..=num {
        let line = lines.next().unwrap().unwrap();
        let vals: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let i = vals[0];
        let j = vals[1];
        p[k - 1] = i;
        if k == num {
            p[k] = j;
        }
    }

    let result = matrix_chain_order(num, &p);
    println!("{}", result);
}

fn matrix_chain_order(num: usize, p: &Vec<usize>) -> usize {
    let mut m = vec![vec![0; num+1]; num+1];
    for l in 2..=num {
        for i in 1..=(num - l + 1) {
            let j = i + l - 1;
            m[i][j] = usize::MAX;
            for k in i..j {
                let q = m[i][k] + m[k+1][j] + p[i-1] * p[k] * p[j];
                if q < m[i][j] {
                    m[i][j] = q;
                }
            }
        }
    }
    m[1][num]
}
