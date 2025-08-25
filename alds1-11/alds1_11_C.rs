use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut graph = vec![vec![0; n]; n];
    let mut status = vec![1; n];
    let mut distance = vec![-1; n];
    let mut bfs_queue = vec![0; n+1];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let node_id = nums.next().unwrap();
        let edge_num = nums.next().unwrap();
        for _ in 0..edge_num {
            let to = nums.next().unwrap();
            graph[node_id-1][to-1] = 1;
        }
    }

    bfs_queue[0] = 0;
    distance[0] = 0;
    let mut tail = 0;
    for head in 0..n {
        let current = bfs_queue[head];
        for next in 0..n {
            if graph[current][next] == 1 && status[next] == 1 {
                status[next] = 0;
                distance[next] = distance[current] + 1;
                tail += 1;
                bfs_queue[tail] = next;
            }
        }
        status[current] = -1;
    }

    for i in 0..n {
        println!("{} {}", i+1, distance[i]);
    }
}
