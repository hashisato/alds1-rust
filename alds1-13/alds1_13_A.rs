use std::io::{self, BufRead};

const N: usize = 8;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut data = vec![vec![-1; N]; N];
    let mut row = vec![0; N];
    let mut col = vec![1; N];
    let mut dpos = vec![1; 2 * N - 1];
    let mut dneg = vec![1; 2 * N - 1];
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        data[x][y] = 0;
    }

    put_queen(
        0,
        &mut data,
        &mut row,
        &mut col,
        &mut dpos,
        &mut dneg,
    );
}

fn put_queen(
    i: usize,
    data: &mut Vec<Vec<i32>>,
    row: &mut Vec<usize>,
    col: &mut Vec<usize>,
    dpos: &mut Vec<usize>,
    dneg: &mut Vec<usize>,
) {
    if i == N {
        print_board(data, row);
        return;
    }
    for j in 0..N {
        let dneg_idx = i + N - 1 - j;
        if col[j] == 0 || dpos[i + j] == 0 || dneg[dneg_idx] == 0 {
            continue;
        }
        row[i] = j;
        col[j] = 0;
        dpos[i + j] = 0;
        dneg[dneg_idx] = 0;
        put_queen(i + 1, data, row, col, dpos, dneg);
        col[j] = 1;
        dpos[i + j] = 1;
        dneg[dneg_idx] = 1;
    }
}

fn print_board(data: &Vec<Vec<i32>>, row: &Vec<usize>) {    
    for i in 0..N {
        for j in 0..N {
            if data[i][j] == 0 {
                if row[i] != j {
                    return;
                }
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            if row[i] == j {
                print!("Q");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
