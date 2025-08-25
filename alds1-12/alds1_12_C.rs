use std::io::{self, BufRead};

const INF: i32 = std::i32::MAX;

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
        let u = nums.next().unwrap() as usize;
        let k = nums.next().unwrap() as usize;
        last_node = u;
        for _ in 0..k {
            let v = nums.next().unwrap() as usize;
            let c = nums.next().unwrap();
            edges.push((u, v, c));
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
