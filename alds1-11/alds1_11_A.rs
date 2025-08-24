use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut data = vec![vec![0; n]; n];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let inid = nums.next().unwrap();
        let inum = nums.next().unwrap();
        for _ in 0..inum {
            let indata = nums.next().unwrap();
            data[inid-1][indata-1] = 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            if j > 0 {
                print!(" ");
            }
            print!("{}", data[i][j]);
        }
        println!();
    }
}
