use std::io::{self, BufRead};

struct Node {
    key: i32,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let keys_line = lines.next().unwrap().unwrap();
    let keys: Vec<i32> = keys_line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut nodes: Vec<Node> = Vec::with_capacity(n + 1);
    nodes.push(Node { key: 0, parent: None, left: None, right: None });

    for i in 1..=n {
        let parent = if i/2 > 0 { Some(i/2) } else { None };
        let left = if i*2 <= n { Some(i*2) } else { None };
        let right = if i*2+1 <= n { Some(i*2+1) } else { None };
        nodes.push(Node {
            key: keys[i - 1],
            parent,
            left,
            right,
        });
    }

    for i in 1..=n {
        print!("node {}: key = {}", i, nodes[i].key);
        if let Some(p) = nodes[i].parent {
            print!(", parent key = {}", nodes[p].key);
        }
        if let Some(l) = nodes[i].left {
            print!(", left key = {}", nodes[l].key);
        }
        if let Some(r) = nodes[i].right {
            print!(", right key = {}", nodes[r].key);
        }
        print!(",\n");
    }
}
