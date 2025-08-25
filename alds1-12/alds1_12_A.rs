use std::io::{self, BufRead};

const INF: i32 = std::i32::MAX;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut data = vec![vec![-1; num]; num];
    for i in 0..num {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        for j in 0..num {
            data[i][j] = row[j];
        }
    }

    let mut d = vec![INF; num];
    let mut pi = vec![-1; num];
    let mut color = vec![2; num];
    d[0] = 0;

    loop {
        let mut mincost = INF;
        let mut u = -1;
        for i in 0..num {
            if color[i] != 0 && d[i] < mincost {
                mincost = d[i];
                u = i as i32;
            }
        }
        if u == -1 {
            break;
        }
        color[u as usize] = 0;
        for j in 0..num {
            if color[j] != 0 && data[u as usize][j] != -1 {
                if data[u as usize][j] < d[j] {
                    pi[j] = u;
                    d[j] = data[u as usize][j];
                    color[j] = 1;
                }
            }
        }
    }

    let result: i32 = d.iter().sum();
    println!("{}", result);
}
