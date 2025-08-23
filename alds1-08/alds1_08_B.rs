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
    let mut nodes: Vec<Node> = Vec::new();
    let mut root: Option<usize> = None;

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        match parts[0] {
            "insert" => {
                let key: i32 = parts[1].parse().unwrap();
                insert(&mut nodes, &mut root, key);
            }
            "find" => {
                let key: i32 = parts[1].parse().unwrap();
                if let Some(r) = root {
                    if tree_search(&nodes, r, key).is_some() {
                        println!("yes");
                    } else {
                        println!("no");
                    }
                } else {
                    println!("no");
                }
            }
            "print" => {
                if let Some(r) = root {
                    inorder(&nodes, r);
                    println!();
                    preorder(&nodes, r);
                    println!();
                }
            }
            _ => {}
        }
    }
}

fn insert(nodes: &mut Vec<Node>, root: &mut Option<usize>, key: i32) {
    let mut y: Option<usize> = None;
    let mut x = *root;
    while let Some(idx) = x {
        y = x;
        if key < nodes[idx].key {
            x = nodes[idx].left;
        } else {
            x = nodes[idx].right;
        }
    }
    let z = nodes.len();
    nodes.push(Node {
        left: None,
        right: None,
        parent: y,
        key,
    });
    if let Some(yidx) = y {
        if key < nodes[yidx].key {
            nodes[yidx].left = Some(z);
        } else {
            nodes[yidx].right = Some(z);
        }
    } else {
        *root = Some(z);
    }
}

fn tree_search(nodes: &Vec<Node>, idx: usize, key: i32) -> Option<usize> {
    if nodes[idx].key == key {
        return Some(idx);
    }
    if key < nodes[idx].key {
        if let Some(left) = nodes[idx].left {
            tree_search(nodes, left, key)
        } else {
            None
        }
    } else {
        if let Some(right) = nodes[idx].right {
            tree_search(nodes, right, key)
        } else {
            None
        }
    }
}

fn inorder(nodes: &Vec<Node>, idx: usize) {
    if let Some(left) = nodes[idx].left {
        inorder(nodes, left);
    }
    print!(" {}", nodes[idx].key);
    if let Some(right) = nodes[idx].right {
        inorder(nodes, right);
    }
}

fn preorder(nodes: &Vec<Node>, idx: usize) {
    print!(" {}", nodes[idx].key);
    if let Some(left) = nodes[idx].left {
        preorder(nodes, left);
    }
    if let Some(right) = nodes[idx].right {
        preorder(nodes, right);
    }
}
