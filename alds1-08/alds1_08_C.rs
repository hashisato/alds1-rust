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
            "delete" => {
                let key: i32 = parts[1].parse().unwrap();
                if let Some(z) = root.and_then(|r| tree_search(&nodes, r, key)) {
                    tree_delete(&mut nodes, &mut root, z);
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

fn tree_minimum(nodes: &Vec<Node>, mut idx: usize) -> usize {
    while let Some(left) = nodes[idx].left {
        idx = left;
    }
    idx
}

fn tree_successor(nodes: &Vec<Node>, idx: usize) -> Option<usize> {
    if let Some(right) = nodes[idx].right {
        Some(tree_minimum(nodes, right))
    } else {
        let mut x = idx;
        let mut y = nodes[x].parent;
        while let Some(yidx) = y {
            if Some(x) != nodes[yidx].left {
                x = yidx;
                y = nodes[yidx].parent;
            } else {
                return y;
            }
        }
        None
    }
}

fn transplant(nodes: &mut Vec<Node>, root: &mut Option<usize>, u: usize, v: Option<usize>) {
    let parent = nodes[u].parent;
    if parent.is_none() {
        *root = v;
    } else {
        let pidx = parent.unwrap();
        if Some(u) == nodes[pidx].left {
            nodes[pidx].left = v;
        } else {
            nodes[pidx].right = v;
        }
    }
    if let Some(vidx) = v {
        nodes[vidx].parent = parent;
    }
}

fn tree_delete(nodes: &mut Vec<Node>, root: &mut Option<usize>, z: usize) {
    if nodes[z].left.is_none() {
        transplant(nodes, root, z, nodes[z].right);
    } else if nodes[z].right.is_none() {
        transplant(nodes, root, z, nodes[z].left);
    } else {
        let y = tree_minimum(nodes, nodes[z].right.unwrap());
        let y_parent = nodes[y].parent;
        if y_parent != Some(z) {
            transplant(nodes, root, y, nodes[y].right);
            nodes[y].right = nodes[z].right;
            if let Some(ridx) = nodes[y].right {
                nodes[ridx].parent = Some(y);
            }
        }
        transplant(nodes, root, z, Some(y));
        nodes[y].left = nodes[z].left;
        if let Some(lidx) = nodes[y].left {
            nodes[lidx].parent = Some(y);
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
