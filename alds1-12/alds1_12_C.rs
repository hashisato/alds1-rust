use std::io::{self, BufRead};

const INF: i32 = 100000000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let num: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut edges = Vec::new();
    let mut d = vec![INF; num];
    let mut color = vec![0; num];

    let mut last_node = 0;
    for _ in 0..num {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n1 = nums.next().unwrap() as usize;
        let n2 = nums.next().unwrap() as usize;
        last_node = n1;
        for _ in 0..n2 {
            let n3 = nums.next().unwrap() as usize;
            let n4 = nums.next().unwrap();
            edges.push((n1, n3, n4));
        }
    }

    d[0] = 0;
    color[last_node] = 1;

    loop {
        color[last_node] = 0;
        let mut updated = false;
        for &(from, to, cost) in &edges {
            if d[from] + cost < d[to] {
                d[to] = d[from] + cost;
                color[last_node] = 1;
                updated = true;
            }
        }
        if color[last_node] == 0 {
            break;
        }
    }

    for i in 0..num {
        println!("{} {}", i, d[i]);
    }
}
