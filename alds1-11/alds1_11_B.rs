use std::io::{self, BufRead};

fn dfs(
    node: usize,
    graph: &Vec<Vec<usize>>,
    status: &mut Vec<i32>,
    discovered: &mut Vec<usize>,
    finished: &mut Vec<usize>,
    time: &mut usize,
) {
    status[node] = 0;
    *time += 1;
    discovered[node] = *time;
    for &next in &graph[node] {
        if status[next] == 1 {
            dfs(next, graph, status, discovered, finished, time);
        }
    }
    status[node] = -1;
    *time += 1;
    finished[node] = *time;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut graph = vec![vec![]; n];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let node_id = nums.next().unwrap();
        let edge_num = nums.next().unwrap();
        for _ in 0..edge_num {
            let to = nums.next().unwrap();
            graph[node_id - 1].push(to - 1);
        }
    }

    let mut status = vec![1; n];
    let mut discovered = vec![0; n];
    let mut finished = vec![0; n];
    let mut time = 0;

    for i in 0..n {
        if status[i] == 1 {
            dfs(i, &graph, &mut status, &mut discovered, &mut finished, &mut time);
        }
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, discovered[i], finished[i]);
    }
}
