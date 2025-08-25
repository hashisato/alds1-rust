use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first = lines.next().unwrap().unwrap();
    let mut first_iter = first.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let n = first_iter.next().unwrap();
    let m = first_iter.next().unwrap();

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();
        graph[a].push(b);
        graph[b].push(a);
    }

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut queries = vec![];
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let u = nums.next().unwrap();
        let v = nums.next().unwrap();
        queries.push((u, v));
    }

    let mut comp = vec![0; n];
    let mut group = 1;
    for i in 0..n {
        if comp[i] == 0 {
            dfs(i, group, &graph, &mut comp);
            group += 1;
        }
    }

    for (u, v) in queries {
        if comp[u] == comp[v] {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn dfs(u: usize, group: usize, graph: &Vec<Vec<usize>>, comp: &mut Vec<usize>) {
    comp[u] = group;
    for &v in &graph[u] {
        if comp[v] == 0 {
            dfs(v, group, graph, comp);
        }
    }
}
