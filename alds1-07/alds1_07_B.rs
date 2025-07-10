use std::io::{self, BufRead};
use std::str::FromStr;

struct Node {
    id: usize,
    parent: isize,
    left_child: isize,
    right_child: isize,
    sibling: isize,
    degree: usize,
    depth: usize,
    height: usize,
    kind: String,
}

fn main() {
    let n: usize = read_usize();
    let mut nodes: Vec<Node> = Vec::new();

    for i in 0..n {
        nodes.push(Node {
            id: i,
            parent: -1,
            left_child: -1,
            right_child: -1,
            sibling: -1,
            degree: 0,
            depth: 0,
            height: 0,
            kind: String::new(),
        });
    }

    for _ in 0..n {
        let (id, left, right) = read_node();
        nodes[id].left_child = left;
        nodes[id].right_child = right;

        let mut degree = 0;
        if left != -1 { degree += 1; }
        if right != -1 { degree += 1; }
        nodes[id].degree = degree;

        if left != -1 {
            nodes[left as usize].parent = id as isize;
        }
        if right != -1 {
            nodes[right as usize].parent = id as isize;
        }
    }

    for i in 0..n {
        let left = nodes[i].left_child;
        let right = nodes[i].right_child;
        
        if left != -1 && right != -1 {
            nodes[left as usize].sibling = right;
            nodes[right as usize].sibling = left;
        }
    }

    for i in 0..n {
        let mut depth = 0;
        let mut current = nodes[i].parent;
        while current != -1 {
            depth += 1;
            current = nodes[current as usize].parent;
        }
        nodes[i].depth = depth;
    }

    for i in 0..n {
        nodes[i].height = calculate_height(&nodes, i);
    }

    for i in 0..n {
        if nodes[i].parent == -1 {
            nodes[i].kind = "root".to_string();
        } else if nodes[i].degree == 0 {
            nodes[i].kind = "leaf".to_string();
        } else {
            nodes[i].kind = "internal node".to_string();
        }
    }

    for i in 0..n {
        println!("node {}: parent = {}, sibling = {}, degree = {}, depth = {}, height = {}, {}",
            nodes[i].id, nodes[i].parent, nodes[i].sibling, 
            nodes[i].degree, nodes[i].depth, nodes[i].height, nodes[i].kind);
    }
}

fn calculate_height(nodes: &Vec<Node>, node_id: usize) -> usize {
    let left = nodes[node_id].left_child;
    let right = nodes[node_id].right_child;
    
    let mut max_height = 0;
    
    if left != -1 {
        let left_height = calculate_height(nodes, left as usize);
        max_height = max_height.max(left_height + 1);
    }
    
    if right != -1 {
        let right_height = calculate_height(nodes, right as usize);
        max_height = max_height.max(right_height + 1);
    }
    
    max_height
}

fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_node() -> (usize, isize, isize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words: Vec<&str> = input.trim().split_whitespace().collect();
    
    let id: usize = words[0].parse().unwrap();
    let left: isize = words[1].parse().unwrap();
    let right: isize = words[2].parse().unwrap();
    
    (id, left, right)
}